export interface ImageFile {
  id: string;
  name: string;
  path: string;
  thumbnail: string; // base64 data URL
  width: number;
  height: number;
}

export interface Region {
  id: string;
  x: number;
  y: number;
  width: number;
  height: number;
  label?: string;
}

export interface RegionResult {
  regionId: string;
  text: string;
  columns: string[][];
}

export interface OutputCard {
  imageId: string;
  imageName: string;
  results: RegionResult[];
}
