#![deny(warnings)]
#![allow(incomplete_features)]
#![feature(generic_associated_types)]
#![feature(adt_const_params)]
#![feature(generic_arg_infer)]
#![feature(rustc_attrs)]
#![feature(marker_trait_attr)]

/// In this demo, we have a bank, a bank admin, and two accounts at the bank.
/// The first account is going to send some money over to the second account.
/// The first account is *then* going to close their account (which will cash out their remaining balance).
/// The act of closing an account will require a bank admin's approval (which is granted after 5 seconds).
///
/// Here we see advanced policy definitions that allow us to safely build "send money" and
/// "close account" handlers using primitives (like withdraw/deposit) in our service.
///
/// We imagine that a new developer is tasked with writing the send_money handler and close_account handler.
/// Notice that a developer writing these features doesn't even have to be aware of what permissions to check
/// because dacquiri will enforce that the _correct_ permissions are tested when the privileged functionality
/// (withdraw, deposit, close_account) are called. The developer can then add the appropriate checks until dacquiri
/// is satisfied and their app - secure.


use std::time::Duration;
use dacquiri::prelude::*;
use crate::bank::{AccountID, BankAdmin, BankHandle, BankResult};

use crate::bank::attributes::*;
use crate::bank::policies::*;

mod bank;
mod prepare_demo;

const SEND_MONEY_FEE: u64 = 3;
const ADMIN_PASSWORD: &'static str = "supersecurep@ssword123!";
const ACCOUNT_ONE_PASSWORD: &'static str = "password123";
const ACCOUNT_TWO_PASSWORD: &'static str = "hunter2";

fn main() -> BankResult<()> {
    let (
        bank,
        bank_admin,
        account_one,
        account_two
    ) = prepare_demo::prepare_demo();

    let mut sender = account_one
        .into_entity::<"account">()
        .add_entity::<_, "bank">(bank.clone())?
        .prove_with_resource::<NotFrozen<_, _>, "account", "bank">()?
        .prove_with_resource_and_context::<Authenticated<_, _>, "account", "bank">(ACCOUNT_ONE_PASSWORD)?;

    let mut receiver = account_two
        .into_entity::<"account">()
        .add_entity::<_, "bank">(bank.clone())?
        .prove_with_resource::<NotFrozen<_, _>, "account", "bank">()?;

    send_money_handler(
        &mut sender,
        &mut receiver,
        12
    )?;

    let _ = create_account_handler(bank.clone(), bank_admin.clone())?;
    let leftover_balance = close_account_handler(sender, bank_admin)?;

    println!("${leftover_balance} is your leftover balance. Have a great day!");

    Ok(())
}

fn send_money_handler(
    sender: &mut impl AuthenticatedAccountPolicy,
    receiver: &mut impl ActiveAccountPolicy,
    amount: u64
) -> BankResult<()> {
    let _ = sender.withdraw(amount + SEND_MONEY_FEE)?;

    match receiver.deposit(amount) {
        Ok(_) => {
            println!("Sent ${amount} successfully - charged ${SEND_MONEY_FEE} for processing.");
            Ok(())
        },
        Err(_) => {
            println!("Sending ${amount} failed - rolling back.");
            sender.deposit(amount + SEND_MONEY_FEE)
        }
    }
}

fn create_account_handler(
    bank_handle: BankHandle,
    bank_admin: BankAdmin
) -> BankResult<AccountID> {
    println!("Creating account");

    let mut authorized_account_closing = bank_handle
        .into_entity::<"bank">()
        .add_entity::<_, "admin">(bank_admin)?
        .prove_with_resource::<Assigned<_, _>, "admin", "bank">()?
        .prove_with_context::<Authorized<_, _>, "admin">(ADMIN_PASSWORD)?;

    Ok(authorized_account_closing.create_account("newpassword123"))
}

fn close_account_handler(
    closing_account: impl AuthenticatedAccountPolicy,
    bank_admin: BankAdmin
) -> BankResult<u64> {
    println!("Submitted to bank admin for approval.");
    // simulate waiting on bank admin to approve
    std::thread::sleep(Duration::from_secs(5));

    let authorized_account_closing = closing_account
        .add_entity::<_, "admin">(bank_admin)?
        .prove_with_context::<Authorized<_, _>, "admin">(ADMIN_PASSWORD)?
        .prove_with_resource::<Assigned<_, _>, "admin", "bank">()?;

    println!("Account closing - Approved.");

    authorized_account_closing.close_account()
}