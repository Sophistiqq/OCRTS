import { invoke } from '@tauri-apps/api/core';
import type { ImageFile, Region, RegionResult, OutputCard } from './types';

export async function loadImage(path: string): Promise<ImageFile> {
  return invoke<ImageFile>('load_image', { path });
}

export async function processRegion(
  imageId: string,
  region: Region,
  rotation: number,
  blur: number = 0,
  threshold: number = -2
): Promise<RegionResult> {
  return invoke<RegionResult>("process_region", {
    imageId,
    region: { ...region, rotation },
    blur,
    threshold
  });
}

export async function saveResults(
  cards: OutputCard[],
  format: 'txt' | 'csv'
): Promise<string> {
  return invoke<string>('save_results', { cards, format });
}

export async function loadImageFull(path: string): Promise<string> {
  return invoke<string>('load_image_full', { path });
}

export async function preprocessImage(
  path: string,
  blur: number,
  threshold: number
): Promise<string> {
  return invoke<string>('preprocess_image', { path, blur, threshold });
}
