export interface ProcessingSettings {
  blur: number;
  threshold: number; // -1: Otsu, 0-255: Fixed, -2: Disabled
}

export interface ImageFile {
  id: string;
  name: string;
  path: string;
  thumbnail: string; // base64 data URL
  width: number;
  height: number;
  rotation: number;
  processing?: ProcessingSettings;
}

export interface Region {
  id: string;
  x: number;
  y: number;
  width: number;
  height: number;
  label?: string;
  is_numeric?: boolean;
}

export interface OcrCell {
  text: string;
  originalText: string;
  confidence: number;
  manual?: boolean;
}

export interface RegionResult {
  regionId: string;
  text: string;
  columns: OcrCell[][];
}

export interface OutputCard {
  imageId: string;
  imageName: string;
  results: RegionResult[];
}
