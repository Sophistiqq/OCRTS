import { writable } from 'svelte/store';
import type { OutputCard } from '../types';

export const outputCards = writable<OutputCard[]>([]);

export function addCard(card: OutputCard) {
  outputCards.update((cards) => {
    const idx = cards.findIndex((c) => c.imageId === card.imageId);
    if (idx >= 0) {
      const copy = [...cards];
      copy[idx] = card;
      return copy;
    }
    return [...cards, card];
  });
}

export function clearResults() {
  outputCards.set([]);
}
