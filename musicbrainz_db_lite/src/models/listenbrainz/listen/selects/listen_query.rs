use bon::bon;
use sea_query::Asterisk;
use sea_query::Cond;
use sea_query::Condition;
use sea_query::Expr;
use sea_query::Query;
use sea_query::SqliteQueryBuilder;

use crate::models::listenbrainz::listen::Listen;
use crate::models::listenbrainz::listen::ListenIden;
use crate::models::listenbrainz::msid_mapping::MsidMappingIden;
use crate::models::musicbrainz::user::UserIden;

#[bon]
impl Listen {
    /// Create a listen query string
    #[builder]
    pub fn listen_query_string(
        users: &[&str],
        #[builder(default)] mapped: bool,
        #[builder(default)] unmapped: bool,
    ) -> String {
        Query::select()
            .column((ListenIden::Table, Asterisk))
            .from(ListenIden::Table)
            .inner_join(
                UserIden::Table,
                Expr::col((ListenIden::Table, ListenIden::User))
                    .equals((UserIden::Table, UserIden::Name)),
            )
            .left_join(
                MsidMappingIden::Table,
                Condition::all()
                    .add(
                        Expr::col((MsidMappingIden::Table, MsidMappingIden::RecordingMsid))
                            .equals((ListenIden::Table, ListenIden::RecordingMsid)),
                    )
                    .add(
                        Expr::col((MsidMappingIden::Table, MsidMappingIden::User))
                            .equals((UserIden::Table, UserIden::Id)),
                    ),
            )
            .cond_where(
                Condition::all()
                    .add(user_filter(users))
                    .add(mapping_filter(mapped, unmapped)),
            )
            .order_by(
                (ListenIden::Table, ListenIden::ListenedAt),
                sea_query::Order::Desc,
            )
            .to_string(SqliteQueryBuilder)
    }
}

fn user_filter(users: &[&str]) -> sea_query::Condition {
    let mut cond = Cond::any();

    for user in users {
        cond = cond.add(Cond::all().add(Expr::col((ListenIden::Table, ListenIden::User)).eq(*user)))
    }

    cond
}

fn mapping_filter(mapped: bool, unmapped: bool) -> Cond {
    Cond::any()
        .add_option(mapped_filter(mapped))
        .add_option(unmapped_filter(unmapped))
}

fn mapped_filter(mapped: bool) -> Option<Cond> {
    if !mapped {
        return None;
    }

    Some(
        Cond::all()
            .add(Expr::col((MsidMappingIden::Table, MsidMappingIden::RecordingMbid)).is_not_null()),
    )
}

fn unmapped_filter(unmapped: bool) -> Option<Cond> {
    if !unmapped {
        return None;
    }

    Some(
        Cond::all()
            .add(Expr::col((MsidMappingIden::Table, MsidMappingIden::RecordingMbid)).is_null()),
    )
}
