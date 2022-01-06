use crate::impl_principal;
use crate::grant::{Grant, Grantable, PrincipalT};

impl_principal!(User);
pub struct User {
    name: String,
    teams: Vec<u64>,
    grants: Vec<&'static str>
}

pub struct Team {
    name: String,
    team_id: u64
}

impl User {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            teams: Vec::new(),
            grants: Vec::new()
        }
    }

    pub fn add_grant(&mut self, grant: &'static str) -> &mut Self {
        self.grants.push(grant);
        self
    }

    pub fn add_team_id(&mut self, team_id: u64) -> &mut Self {
        self.teams.push(team_id);
        self
    }

    pub fn get_name(&self) -> String { self.name.clone() }
    pub fn is_in_team(&self, team_id: u64) -> bool { self.teams.contains(&team_id) }
}

impl Team {
    pub fn new(name: impl Into<String>, team_id: u64) -> Self {
        Self {
            team_id,
            name: name.into()
        }
    }

    pub fn get_name(&self) -> String { self.name.clone() }
    pub fn get_team_id(&self) -> u64 { self.team_id }
}

impl Grantable<User> for User {}
impl<'team> Grantable<User, &'team Team> for User {}

pub mod grants {
    pub mod add_user {
        use crate::grant::{Grant, GRANT_CHECK_DEFAULT, HasGrant};
        use crate::tests::helpers::{Team, User};

        pub struct AddUserToTeam<'team, const ID: &'static str = GRANT_CHECK_DEFAULT>(&'team Team);

        impl<'team, const ID: &'static str> Grant<ID> for AddUserToTeam<'team, ID> {
            type Principal = User;
            type Resource = &'team Team;
            const NAME: &'static str = "AddUserToTeam";

            fn check_grant(user: &Self::Principal, team: &Self::Resource) -> Result<(), String> {
                if !user.teams.contains(&team.get_team_id()) {
                    return Err(format!("User {} is not a member of team {}.", user.get_name(), team.get_team_id()));
                }

                if !user.grants.contains(&Self::NAME) {
                    return Err(format!("Missing {} grant.", Self::NAME));
                }

                Ok(())
            }

            fn new_with_resource(resource: Self::Resource) -> Self { Self(resource) }

            fn get_resource(&self) -> &Self::Resource { &self.0 }
        }

        pub trait AddUserToTeamGrant<'team, const ID: &'static str = GRANT_CHECK_DEFAULT>: HasGrant<AddUserToTeam<'team, ID>, ID> {
            fn add_user(&self);
        }

        impl<'team, const ID: &'static str, G: HasGrant<AddUserToTeam<'team, ID>, ID>> AddUserToTeamGrant<'team, ID> for G {
            fn add_user(&self) {
                let team: &'team Team = self.get_resource();

                println!("Adding user to the team [{}] '{}'.", team.get_team_id(), team.get_name());
            }
        }
    }

    pub mod change_name {
        use crate::grant::{Grant, HasGrant};
        use crate::tests::helpers::User;

        pub struct ChangeName;

        impl Grant for ChangeName {
            type Principal = User;
            const NAME: &'static str = "ChangeName";

            // give everyone this grant
            fn check_grant(_: &Self::Principal, _: &Self::Resource) -> Result<(), String> { Ok(()) }

            fn new_with_resource(_: Self::Resource) -> Self { Self }

            fn get_resource(&self) -> &Self::Resource { &() }
        }

        pub trait ChangeNameGrant: HasGrant<ChangeName> {
            fn change_name(&mut self, name: impl Into<String>);
        }

        impl<G: HasGrant<ChangeName>> ChangeNameGrant for G {
            fn change_name(&mut self, name: impl Into<String>) {
                self.get_principal_mut().name = name.into();
            }
        }
    }

    pub mod admin {
        use crate::grant::{Grant, HasGrant, PrincipalT};
        use crate::tests::helpers::User;

        pub struct AdminOne;
        pub struct AdminTwo;

        impl Grant for AdminOne {
            type Principal = User;
            const NAME: &'static str = "AdminOne";

            fn check_grant(principal: &Self::Principal, resource: &Self::Resource) -> Result<(), String> {
                Ok(())
            }

            fn new_with_resource(_: Self::Resource) -> Self { Self }

            fn get_resource(&self) -> &Self::Resource { &() }
        }

        impl Grant for AdminTwo {
            type Principal = User;
            const NAME: &'static str = "AdminTwo";

            fn check_grant(principal: &Self::Principal, resource: &Self::Resource) -> Result<(), String> {
                Ok(())
            }

            fn new_with_resource(_: Self::Resource) -> Self { Self }

            fn get_resource(&self) -> &Self::Resource { &() }
        }

        pub trait BigAdmin: HasGrant<AdminOne> + HasGrant<AdminTwo> {
            fn mogul_moves(&self);
        }

        impl<G: HasGrant<AdminOne> + HasGrant<AdminTwo>> BigAdmin for G {
            fn mogul_moves(&self) {
                let user: &User = self.get_principal();
                println!("{} is a big admin boi", user.get_name());
            }
        }
    }
}