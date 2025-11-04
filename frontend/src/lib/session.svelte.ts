import { LobbyState, type Card, type Session, type SessionInformation } from '$lib';

export let session: Session = $state({ info: null! });

export function initSessionByHosting(lobbyId: string, hostName: string, cards: Card[]) {
    session.info = {
        lobbyId,
        userName: hostName,
        isHost: true,
        hostName,
        players: [],
        cards,
        state: LobbyState.WaitingForPlayers,
        correctAnswers: [],
    };
}

export function initSessionByJoining(info: SessionInformation) {
    session.info = info;
}
