// For ClientEvent (using a String Enum)
// This is the most direct and idiomatic translation for mapping a name to a specific string value.
export enum ClientEvent {
    HostLobby = 'hostLobby',
    JoinLobby = 'joinLobby',
}

export enum ServerEvent {
    NameAlreadyTaken = 'nameAlreadyTaken',
}

export interface HostLobbyRequest {
    hostName: string;

    cards: string[];
}
