// place files you want to import through the `$lib` alias in this folder.

import { io, Socket } from 'socket.io-client';
import type { ClientToServerEvents, ServerToClientEvents } from './socket_event_types';
import { session } from './session.svelte';

export interface ServerCard {
    id: number;
    description: string;
}

export interface Card extends ServerCard {
    color: string;
    borderColor: string;
    textColor: string;
}

export type DraggedItemInfo = {
    item: Card | null;
    source: 'palette' | 'grid' | null;
    sourceIndex: number;
};

export interface SessionInformation {
    lobbyId: string;
    userName: string;
    isHost: boolean;
    hostName: string;
    /**
     * Includes EVERY player, including yourself, except the host
     */
    players: string[];
    cards: Card[];
    state: LobbyState;
}

export enum LobbyState {
    /// The lobby is waiting for players to join - started by the host
    WaitingForPlayers,

    CraftingBoards,

    /// The lobby is currently in progress. No more players can join
    InProgress,

    /// The lobby has ended
    Completed,
}

export type Session = { info: SessionInformation | null };

export const COLOR_THEMES = [
    {
        color: 'bg-primary-700/70',
        borderColor: 'border-primary-400',
        textColor: 'text-primary-400',
    },
    {
        color: 'bg-secondary-700/70',
        borderColor: 'border-secondary-400',
        textColor: 'text-secondary-400',
    },
    {
        color: 'bg-tertiary-700/70',
        borderColor: 'border-tertiary-400',
        textColor: 'text-tertiary-400',
    },
    {
        color: 'bg-warning-700/70',
        borderColor: 'border-warning-400',
        textColor: 'text-warning-400',
    },
    { color: 'bg-error-700/70', borderColor: 'border-error-400', textColor: 'text-error-400' },
];

export const socket: Socket<ServerToClientEvents, ClientToServerEvents> =
    io('http://localhost:3000/');

socket.on('disconnect', (reason, details) => {
    // the reason of the disconnection, for example "transport error"
    console.log(reason);

    if (!details) return;

    // the low-level reason of the disconnection, for example "xhr post error"
    console.log(details);
});

export const BACKEND_URL = 'http://localhost:3000';
export const FRONTEND_URL = 'http://localhost:5173';
