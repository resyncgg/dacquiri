use crate::attributes::*;
use crate::models::*;
use dacquiri::prelude::policy;
type Files = String;


#[policy(
    entities = (
        user: User,
        team: Team,
    ),
    constraints = (
        user is EnabledUser,
        team is EnabledTeam,
        user is MemberOfTeam for team,
    )
)]
pub trait TeamMember {
    fn get_files(&self) -> Files {
        let this_team: &Team = self.get_entity::<_, team>().unwrap();
        format!("<files for {}>", this_team.name)
    }
    fn write_files(&self, files: Files) {
        let this_team: &Team = self.get_entity::<_, team>().unwrap();

        println!("Writing files to team {}: {}", this_team.name, files);
    }
}

#[policy(
    entities = (
        user: User,
        team_a: Team,
        team_b: Team,
    ),
    policies = (
        TeamMember(user, team_a),
        TeamMember(user, team_b),
    )
)]
pub trait MultiTeamMember {
    fn transfer_data(&self) {
        let team_a_files = <Self as TeamMember<user, team_a>>::get_files(self);

        <Self as TeamMember<user, team_b>>::write_files(self, team_a_files);
    }
}
