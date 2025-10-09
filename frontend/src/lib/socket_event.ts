// --- REQUEST PAYLOADS (Client -> Server) ---

type Acknowledgement<T> =
    | {
          success: true;
          data: T;
      }
    | { success: false; error: string };

export interface HostLobbyRequest {
    hostName: string;
    cards: string[];
}

export interface JoinLobbyRequest {
    lobbyId: string;
    userName: string;
}

// --- ACKNOWLEDGMENT PAYLOADS (Server -> Client) ---

export type HostLobbyAck = Acknowledgement<{
    lobbyId: string;
}>;

export type JoinLobbyAck = Acknowledgement<{
    lobbyId: string;
}>;

export interface ClientToServerEvents {
    // Event: 'hostLobby'
    // Arguments: [HostLobbyRequest, (ack: HostLobbyAck) => void]
    hostLobby: (data: HostLobbyRequest, callback: (ack: HostLobbyAck) => void) => void;

    // Event: 'joinLobby'
    // Arguments: [JoinLobbyRequest, (ack: JoinLobbyAck) => void]
    joinLobby: (data: JoinLobbyRequest, callback: (ack: JoinLobbyAck) => void) => void;
}

export interface ServerToClientEvents {
    // Server pushes this event to notify clients a new user joined
    userJoined: (userName: string) => void;
}
