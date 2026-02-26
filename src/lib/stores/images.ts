import { writable, derived } from 'svelte/store';
import type { ImageFile, Region } from '../types';

// The queue of uploaded images
export const imageQueue = writable<ImageFile[]>([]);

// Regions per image: imageId → Region[]
export const regionMap = writable<Record<string, Region[]>>({});

// Which image is currently being edited (by index)
export const currentIndex = writable<number>(0);

export const currentImage = derived(
  [imageQueue, currentIndex],
  ([$queue, $index]) => $queue[$index] ?? null
);

export const totalImages = derived(imageQueue, ($q) => $q.length);

// Actions
export function addImages(images: ImageFile[]) {
  imageQueue.update((q) => {
    const existingPaths = new Set(q.map((i) => i.path));
    const fresh = images.filter((i) => !existingPaths.has(i.path));
    return [...q, ...fresh];
  });
}

export function removeImage(id: string) {
  imageQueue.update((q) => q.filter((i) => i.id !== id));
  regionMap.update((m) => {
    const copy = { ...m };
    delete copy[id];
    return copy;
  });
}

export function reorderImages(from: number, to: number) {
  imageQueue.update((q) => {
    const copy = [...q];
    const [moved] = copy.splice(from, 1);
    copy.splice(to, 0, moved);
    return copy;
  });
}

export function setRegions(imageId: string, regions: Region[]) {
  regionMap.update((m) => ({ ...m, [imageId]: regions }));
}

export function addRegion(imageId: string, region: Region) {
  regionMap.update((m) => ({
    ...m,
    [imageId]: [...(m[imageId] ?? []), region],
  }));
}

export function removeRegion(imageId: string, regionId: string) {
  regionMap.update((m) => ({
    ...m,
    [imageId]: (m[imageId] ?? []).filter((r) => r.id !== regionId),
  }));
}

export function rotateImage(id: string, delta: number) {
  imageQueue.update((q) =>
    q.map((i) =>
      i.id === id ? { ...i, rotation: (i.rotation + delta + 360) % 360 } : i
    )
  );
}
