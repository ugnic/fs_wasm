/* tslint:disable */
/* eslint-disable */
/**
* @param {string} file_path
*/
export function open_test(file_path: string): void;
/**
*/
export class Archive {
  free(): void;
}
/**
*/
export class CentralDirectoryEntry {
  free(): void;
}
/**
*/
export class DataDescriptor {
  free(): void;
}
/**
*/
export class EndOfCentralDirectory {
  free(): void;
}
/**
*/
export class File {
  free(): void;
}
/**
*/
export class FileEntry {
  free(): void;
}
/**
*/
export class LocalHeader {
  free(): void;
}
/**
*/
export class Zip {
  free(): void;
/**
* @param {number} seed
*/
  constructor(seed: number);
/**
* @param {string} file_path
*/
  open_test(file_path: string): void;
/**
* @returns {string}
*/
  get_uuid(): string;
/**
* @returns {string}
*/
  get_passwd(): string;
/**
* @returns {number}
*/
  get_compress_level(): number;
/**
* @param {string} passwd
*/
  set_passwd(passwd: string): void;
/**
* @param {string} file_name
* @param {BigInt} file_size
* @param {Uint16Array} last_modified
* @param {Uint8Array} file_raw
*/
  add_file(file_name: string, file_size: BigInt, last_modified: Uint16Array, file_raw: Uint8Array): void;
/**
* @returns {Uint8Array}
*/
  save(): Uint8Array;
/**
* @param {Archive} archive
* @returns {Uint8Array}
*/
  check_archive(archive: Archive): Uint8Array;
}
/**
*/
export class Zip64EndOfCentralDirectoryLocator {
  free(): void;
}
/**
*/
export class Zip64EndOfCentralDirectoryRecord {
  free(): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_localheader_free: (a: number) => void;
  readonly __wbg_centraldirectoryentry_free: (a: number) => void;
  readonly __wbg_endofcentraldirectory_free: (a: number) => void;
  readonly __wbg_datadescriptor_free: (a: number) => void;
  readonly __wbg_zip64endofcentraldirectoryrecord_free: (a: number) => void;
  readonly __wbg_zip64endofcentraldirectorylocator_free: (a: number) => void;
  readonly __wbg_fileentry_free: (a: number) => void;
  readonly __wbg_archive_free: (a: number) => void;
  readonly __wbg_file_free: (a: number) => void;
  readonly __wbg_zip_free: (a: number) => void;
  readonly open_test: (a: number, b: number) => void;
  readonly zip_new: (a: number) => number;
  readonly zip_open_test: (a: number, b: number, c: number) => void;
  readonly zip_get_uuid: (a: number, b: number) => void;
  readonly zip_get_passwd: (a: number, b: number) => void;
  readonly zip_get_compress_level: (a: number) => number;
  readonly zip_set_passwd: (a: number, b: number, c: number) => void;
  readonly zip_add_file: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number) => void;
  readonly zip_save: (a: number, b: number) => void;
  readonly zip_check_archive: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
        