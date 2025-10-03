// place files you want to import through the `$lib` alias in this folder.

export interface Card {
    id: number;
    content: string;
    color: string;
    borderColor: string;
    textColor: string;
}

export type DraggedItemInfo = {
    item: Card | null;
    source: 'palette' | 'grid' | null;
    sourceIndex: number;
};

export const COLOR_THEMES = [
    {
        color: 'bg-primary-700/70',
        borderColor: 'border-primary-400',
        textColor: 'text-primary-400'
    },
    {
        color: 'bg-secondary-700/70',
        borderColor: 'border-secondary-400',
        textColor: 'text-secondary-400'
    },
    {
        color: 'bg-tertiary-700/70',
        borderColor: 'border-tertiary-400',
        textColor: 'text-tertiary-400'
    },
    {
        color: 'bg-warning-700/70',
        borderColor: 'border-warning-400',
        textColor: 'text-warning-400'
    },
    { color: 'bg-error-700/70', borderColor: 'border-error-400', textColor: 'text-error-400' }
];
