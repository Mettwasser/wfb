// --- REQUEST PAYLOADS (Client -> Server) ---

import type { Card, LobbyState, ServerCard } from '$lib';

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
    playerName: string;
}

export interface SubmitBoardRequest {
    lobbyId: string;
    cards: number[];
}

export interface SubmitAnswerRequest {
    lobbyId: string;
    cards: number;
}

// --- ACKNOWLEDGMENT PAYLOADS (Server -> Client) ---

export type HostLobbyAck = Acknowledgement<{
    lobbyId: string;
    cards: ServerCard[];
}>;

export type JoinLobbyAck = Acknowledgement<{
    host: string;
    cards: ServerCard[];
    players: string[];
}>;

export type NextStageAck = Acknowledgement<string>;

export type EmptyAck = Acknowledgement<null>;

export interface ClientToServerEvents {
    // Event: 'hostLobby'
    // Arguments: [HostLobbyRequest, (ack: HostLobbyAck) => void]
    hostLobby: (data: HostLobbyRequest, callback: (ack: HostLobbyAck) => void) => void;

    // Event: 'joinLobby'
    // Arguments: [JoinLobbyRequest, (ack: JoinLobbyAck) => void]
    joinLobby: (data: JoinLobbyRequest, callback: (ack: JoinLobbyAck) => void) => void;

    triggerNextStage: (data: string) => void;

    submitBoard: (data: SubmitBoardRequest, callback: (ack: EmptyAck) => void) => void;

    submitAnswer: (data: SubmitAnswerRequest, callback: (ack: EmptyAck) => void) => void;
}

export interface ServerToClientEvents {
    // Server pushes this event to notify clients a new user joined
    userJoined: (userName: string) => void;

    userLeft: (userName: string) => void;

    lobbyClosed: () => void;

    nextStage: (state: LobbyState) => void;

    boardSubmitted: (userName: string) => void;

    answerSubmitted: (cardId: number) => void;

    winnerDetected: (winners: string[]) => void;
}
