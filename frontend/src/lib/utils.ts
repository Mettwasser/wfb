import { COLOR_THEMES, type Card, type ServerCard } from '$lib';

export function mapServerCardToCard(serverCards: ServerCard[]): Card[] {
    return serverCards.map((card, index) => {
        const theme = COLOR_THEMES[index % COLOR_THEMES.length];
        return {
            ...card,
            ...theme,
        };
    });
}
