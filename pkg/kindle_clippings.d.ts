/* tslint:disable */
/* eslint-disable */
/**
* Parses clipping file content
*
* ```
* use kindle_clippings::parsing::parse_clippings;
* let content = "The Grapes of Wrath (John Steinbeck)\n- Your Highlight on page 12453252 | Added on Friday, 5 July 2024 09:55:52\nHow can we live without our lives ? How will we know it's us without our past ? No. Leave it. Burn it.\n";
* let parsed = parse_clippings(content);
* assert_eq!(1, parsed.len());
* let entry = parsed.get(0).unwrap();
* assert_eq!(Some("John Steinbeck".to_string()), entry.author());
* assert_eq!("The Grapes of Wrath".to_string(), entry.title());
* assert_eq!(Some("12453252".to_string()), entry.page());
* assert_eq!(Some("How can we live without our lives ? How will we know it's us without our past ? No. Leave it. Burn it.".to_string()), entry.content());
* assert_eq!("Friday, 5 July 2024 09:55:52".to_string(), entry.date());
* assert_eq!(None, entry.location());
* ```
* @param {string} content
* @returns {(Entry)[]}
*/
export function parse_clippings(content: string): (Entry)[];
/**
* @param {Entry} e
* @returns {string}
*/
export function exportEntry(e: Entry): string;
/**
*/
export enum ParsingError {
  MalformedLine = 0,
}
/**
*/
export class Entry {
  free(): void;
/**
* @param {string} title
* @param {string | undefined} author
* @param {string} action
* @param {string | undefined} page
* @param {string | undefined} location
* @param {string} date
* @param {string | undefined} [content]
*/
  constructor(title: string, author: string | undefined, action: string, page: string | undefined, location: string | undefined, date: string, content?: string);
/**
* @param {string} sequence
* @returns {boolean}
*/
  author_contains(sequence: string): boolean;
/**
*/
  readonly action: string;
/**
*/
  readonly author: string | undefined;
/**
*/
  readonly content: string | undefined;
/**
*/
  readonly date: string;
/**
*/
  readonly location: string | undefined;
/**
*/
  readonly page: string | undefined;
/**
*/
  readonly title: string;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_entry_free: (a: number) => void;
  readonly entry_new: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number) => number;
  readonly entry_title: (a: number, b: number) => void;
  readonly entry_author: (a: number, b: number) => void;
  readonly entry_action: (a: number, b: number) => void;
  readonly entry_date: (a: number, b: number) => void;
  readonly entry_page: (a: number, b: number) => void;
  readonly entry_location: (a: number, b: number) => void;
  readonly entry_content: (a: number, b: number) => void;
  readonly entry_author_contains: (a: number, b: number, c: number) => number;
  readonly parse_clippings: (a: number, b: number, c: number) => void;
  readonly exportEntry: (a: number, b: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
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
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
