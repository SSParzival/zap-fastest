//! Account-local community roles observed through official group metadata.
use super::{Archive, Result};
use rusqlite::params;
impl Archive {
    pub fn set_community(
        &self,
        chat: &str,
        role: Option<&crate::model::CommunityRole>,
    ) -> Result<()> {
        let role = role
            .map(serde_json::to_string)
            .transpose()
            .map_err(|error| rusqlite::Error::ToSqlConversionFailure(Box::new(error)))?;
        self.connection.execute(
            "UPDATE chats SET community=?2 WHERE id=?1",
            params![chat, role],
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn community_roles_round_trip_and_preserve_locked_children() {
        let archive = Archive::in_memory().unwrap();
        archive.ensure_chat("100-1@g.us", "Fixture parent").unwrap();
        archive.ensure_chat("100-2@g.us", "Fixture child").unwrap();
        archive
            .set_community("100-1@g.us", Some(&crate::model::CommunityRole::Parent))
            .unwrap();
        let member = crate::model::CommunityRole::Member {
            parent: "100-1@g.us".into(),
            announcement: true,
            general: false,
        };
        archive.set_community("100-2@g.us", Some(&member)).unwrap();
        assert_eq!(
            archive.chat("100-2@g.us").unwrap().unwrap().community,
            Some(member.clone())
        );
        archive.set_locked("100-2@g.us", true).unwrap();
        let children = archive.chats().unwrap();
        let child = children
            .iter()
            .find(|chat| chat.id == "100-2@g.us")
            .unwrap();
        assert!(child.locked);
        assert_eq!(child.community, Some(member));
        archive.set_community("100-1@g.us", None).unwrap();
        assert_eq!(archive.chat("100-1@g.us").unwrap().unwrap().community, None);
    }
}
