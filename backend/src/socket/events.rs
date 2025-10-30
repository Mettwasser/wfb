use std::borrow::Cow;

use identconv::camel_strify;

client_events! {
    HostLobby,
    JoinLobby,
    TriggerNextStage,
    SubmitBoard,
}

server_events! {
    UserJoined,
    UserLeft,
    LobbyClosed,
    NextStage,
    BoardSubmitted,
}

macro_rules! client_events {
    ($($variant:ident),* $(,)?) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum ClientEvent {
            $( $variant ),*
        }

        impl From<ClientEvent> for Cow<'static, str> {
            fn from(value: ClientEvent) -> Self {
                match value {
                    $( ClientEvent::$variant => Cow::Borrowed(camel_strify!($variant)) ),*
                }
            }
        }
    };
}

macro_rules! server_events {
    ( $( $variant:ident ),* $(,)? ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum ServerEvent {
            $( $variant ),*
        }

        impl AsRef<str> for ServerEvent {
            fn as_ref(&self) -> &str {
                match self {
                    $( Self::$variant => camel_strify!($variant) ),*
                }
            }
        }
    };
}

use client_events;
use server_events;
