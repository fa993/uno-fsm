/* tslint:disable */
/* eslint-disable */
/**
* @param {string} name
*/
export function greet(name: string): void;
/**
*/
export enum CardType {
  Red,
  Blue,
  Green,
  Yellow,
}
/**
*/
export enum UnoStateType {
  WaitingForDiscard,
  WaitingForDraw,
}
/**
*/
export class UnoCard {
  free(): void;
}
/**
*/
export class UnoGameState {
  free(): void;
/**
* @returns {UnoGameState}
*/
  static new(): UnoGameState;
/**
* @param {number} id
* @returns {string}
*/
  draw(id: number): string;
/**
* @param {number} id
* @returns {string}
*/
  no_card(id: number): string;
/**
* @param {number} id
* @param {number} color
* @param {number} num
* @returns {string}
*/
  discard(id: number, color: number, num: number): string;
/**
* @returns {string}
*/
  output(): string;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_unocard_free: (a: number) => void;
  readonly __wbg_unogamestate_free: (a: number) => void;
  readonly unogamestate_new: () => number;
  readonly unogamestate_draw: (a: number, b: number, c: number) => void;
  readonly unogamestate_no_card: (a: number, b: number, c: number) => void;
  readonly unogamestate_discard: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly unogamestate_output: (a: number, b: number) => void;
  readonly greet: (a: number, b: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
