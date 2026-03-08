/*  docWX: A minimal, performant book-keeping, authoring and documentation tool.
Copyright (C) 2026 argmaxin

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.*/

//& @page("source")
//&  @section(1000)
//& ## Imports
//& We use the following crates here:
//& - `crate::table`: for our prefixes, via a Perfect Hash Function (PHF).
//& - `anyhow`: for errors and diagnostics
//& - `memchr`: for `memchr::memmem`
//& The rest are plain obvious and hence omitted for brevity.
//&   @code
use crate::table;
use anyhow::anyhow;
use memchr::memmem;
use memmap2::Mmap;
use std::collections::BTreeMap;
use std::ffi::OsStr;
use std::fs::File;
use std::path::Path;
//&   @endcode
//&  @endsection

pub struct Parser<'a> {
    src: Mmap,
    prefix: &'static [u8],
    ext: &'a str,
    name: &'a Path,
    dummy: bool,
}

//&  @section(2000)
//& We derive `Eq` trait and use implementations for `PartialEq`, `Ord` and `PartialOrd`.
//& These traits are needed for `BTreeMap` of `std::collections`.
//&   @code
//& We own the buffers inside the String to avoid ownership complications.
#[derive(Eq)]
pub struct Section {
    pub id: usize,
    pub buf: String,
}
//&   @endcode
//&  @endsection

impl PartialEq for Section {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Ord for Section {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.id).cmp(&(other.id))
    }
}

impl PartialOrd for Section {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Page {
    pub path: String,
    pub sections: Vec<Section>,
}

pub struct Document {
    pub pages: Vec<Page>,
}

impl Document {
    pub fn new() -> Self {
        Self { pages: Vec::new() }
    }

    pub fn get_or_create_page_index(&mut self, path: &str) -> usize {
        if let Some(pos) = self.pages.iter().position(|p| p.path == path) {
            return pos;
        }

        let mut path = path.to_string();
        path.push_str(".md");
        self.pages.push(Page {
            path,
            sections: Vec::new(),
        });

        self.pages.len() - 1
    }
}

impl Page {
    pub fn add_section(&mut self, name: &Path, id: usize) -> anyhow::Result<&mut Section> {
        if self.sections.iter().any(|s| s.id == id) {
            eprintln!("error: duplicate section id {} at {}.", id, name.display());
        }

        self.sections.push(Section {
            id,
            buf: String::new(),
        });

        Ok(self.sections.last_mut().unwrap())
    }
}

struct PageFrame {
    page_index: usize,
    section_stack: Vec<usize>,
}

fn parse_page_directive(line: &str) -> Option<String> {
    let s = strip_ws(line);

    if !s.starts_with("@page") {
        return None;
    }

    let rest = s["@page".len()..].trim_start();

    if !rest.starts_with('(') || !rest.ends_with(')') {
        return None;
    }

    let inner = &rest[1..rest.len() - 1].trim();

    if inner.starts_with('"') && inner.ends_with('"') && inner.len() >= 2 {
        Some(inner[1..inner.len() - 1].to_string())
    } else {
        None
    }
}

fn parse_section_directive(line: &str) -> Result<Option<usize>, anyhow::Error> {
    let s = strip_ws(line);

    if !s.starts_with("@section") {
        return Ok(None);
    }

    let rest = s["@section".len()..].trim_start();

    if !rest.starts_with('(') || !rest.ends_with(')') {
        return Ok(None);
    }

    let inner = strip_ws(&rest[1..rest.len() - 1]);

    let id = inner
        .parse::<usize>()
        .map_err(|_| anyhow!("invalid section id"))?;

    Ok(Some(id))
}

//&
pub fn merge_documents(docs: Vec<Document>) -> Option<Document> {
    let mut map: BTreeMap<String, BTreeMap<usize, Section>> = BTreeMap::new();

    for doc in docs {
        for page in doc.pages {
            let page_entry = map.entry(page.path.clone()).or_default();
            let mut has_dup = false;

            for section in page.sections {
                if page_entry.contains_key(&section.id) {
                    eprintln!(
                        "error: duplicate section id {} in page '{}'. Ordering is non-deterministic.",
                        section.id, page.path
                    );
                    has_dup = true;
                }
                page_entry.insert(section.id, section);
            }
            if has_dup {
                return None;
            }
        }
    }

    let pages = map
        .into_iter()
        .map(|(path, sections_map)| {
            let sections = sections_map.into_values().collect();
            Page { path, sections }
        })
        .collect();

    Some(Document { pages })
}

static PREFIX_TABLES: phf::Map<&'static str, &'static str> = table::build_tables();

fn strip_ws(s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut start = 0;
    let mut end = bytes.len();

    while start < end && bytes[start].is_ascii_whitespace() {
        start += 1;
    }
    while end > start && bytes[end - 1].is_ascii_whitespace() {
        end -= 1;
    }

    &s[start..end]
}

impl<'a> Parser<'a> {
    pub fn new(name: &'a Path) -> anyhow::Result<Self> {
        let file = File::open(name)?;
        let mmap_file = unsafe { Mmap::map(&file)? };
        let stem = name.file_stem().and_then(OsStr::to_str);
        let extension = name.extension().and_then(OsStr::to_str);

        let (prefix, extz, dummy) = extension
            .and_then(|ext| {
                let key = format!(".{ext}");
                PREFIX_TABLES.get(key.as_str()).map(|&p| (p, ext, false))
            })
            .or_else(|| stem.and_then(|s| PREFIX_TABLES.get(s).map(|&p| (p, "", false))))
            .unwrap_or(("dummy", "", true));

        Ok(Self {
            src: mmap_file,
            prefix: prefix.as_bytes(),
            dummy,
            name,
            ext: extz,
        })
    }

    pub fn parse(&mut self) -> Result<Document, anyhow::Error> {
        let mut doc = Document::new();

        if self.dummy {
            return Ok(doc);
        }

        let mut stack: Vec<PageFrame> = Vec::new();
        let mut code_snapshot: Option<Vec<(usize, Vec<usize>)>> = None;
        let mut capture_cursor: usize = 0;
        let mut code_buffer = String::new();

        let src = self.src.as_ref();
        let plen = self.prefix.len();
        let mut search_start = 0;

        while let Some(hit) = memmem::find(&src[search_start..], self.prefix) {
            let absolute = search_start + hit;

            if absolute > 0 && src[absolute - 1] == b'\\' {
                search_start = absolute + plen;
                continue;
            }

            if let Some(snapshot) = &code_snapshot {
                let captured = &src[capture_cursor..absolute];
                code_buffer.push_str(std::str::from_utf8(captured)?);

                let line_start = absolute + plen;

                if line_start >= src.len() {
                    return Err(anyhow!("unterminated @code block"));
                }

                let line_end = src[line_start..]
                    .iter()
                    .position(|&b| b == b'\n')
                    .map(|p| line_start + p)
                    .unwrap_or(src.len());

                let mut line = &src[line_start..line_end];

                if line.ends_with(b"\r") {
                    line = &line[..line.len() - 1];
                }

                let line_str = std::str::from_utf8(line)?;

                if strip_ws(line_str) == "@endcode" {
                    for (page_index, sections) in snapshot {
                        let page = &mut doc.pages[*page_index];

                        for &sec_index in sections {
                            page.sections[sec_index].buf.push_str("```");
                            page.sections[sec_index].buf.push_str(self.ext);
                            page.sections[sec_index].buf.push('\n');
                            page.sections[sec_index].buf.push_str(&code_buffer);
                            page.sections[sec_index].buf.push_str("```\n");
                        }
                    }

                    code_snapshot = None;
                    code_buffer.clear();
                    search_start = if line_end < src.len() {
                        line_end + 1
                    } else {
                        src.len()
                    };
                    continue;
                }

                capture_cursor = line_end + 1;
                search_start = if line_end < src.len() {
                    line_end + 1
                } else {
                    src.len()
                };
                continue;
            }

            let line_start = absolute + plen;

            if line_start >= src.len() {
                break;
            }

            let line_end = memchr::memchr(b'\n', &src[line_start..])
                .map(|p| line_start + p)
                .unwrap_or(src.len());

            let mut line = &src[line_start..line_end];

            if line.ends_with(b"\r") {
                line = &line[..line.len() - 1];
            }

            let line_str = std::str::from_utf8(line)?;

            if let Some(path) = parse_page_directive(line_str) {
                let page_index = doc.get_or_create_page_index(&path);

                stack.push(PageFrame {
                    page_index,
                    section_stack: Vec::new(),
                });

                search_start = if line_end < src.len() {
                    line_end + 1
                } else {
                    src.len()
                };
                continue;
            }

            if strip_ws(line_str) == "@endpage" {
                let frame = stack.pop().ok_or_else(|| anyhow!("unexpected @endpage"))?;

                if !frame.section_stack.is_empty() {
                    return Err(anyhow!("page closed with open sections"));
                }

                search_start = if line_end < src.len() {
                    line_end + 1
                } else {
                    src.len()
                };
                continue;
            }

            if let Some(id) = parse_section_directive(line_str)? {
                let frame = stack
                    .last_mut()
                    .ok_or_else(|| anyhow!("section outside page"))?;

                let page = &mut doc.pages[frame.page_index];
                page.add_section(self.name, id)?;
                let sec_index = page.sections.len() - 1;

                frame.section_stack.push(sec_index);

                search_start = if line_end < src.len() {
                    line_end + 1
                } else {
                    src.len()
                };
                continue;
            }

            if strip_ws(line_str) == "@endsection" {
                let frame = stack
                    .last_mut()
                    .ok_or_else(|| anyhow!("unexpected @endsection"))?;

                if frame.section_stack.pop().is_none() {
                    return Err(anyhow!("no open section to close"));
                }

                search_start = if line_end < src.len() {
                    line_end + 1
                } else {
                    src.len()
                };
                continue;
            }

            if strip_ws(line_str) == "@code" {
                if code_snapshot.is_some() {
                    return Err(anyhow!("nested @code not allowed"));
                }

                let snapshot = stack
                    .iter()
                    .map(|f| (f.page_index, f.section_stack.clone()))
                    .collect::<Vec<_>>();

                code_snapshot = Some(snapshot);
                code_buffer.clear();

                capture_cursor = line_end + 1;
                search_start = if line_end < src.len() {
                    line_end + 1
                } else {
                    src.len()
                };
                continue;
            }

            if !stack.is_empty() {
                for frame in &stack {
                    let page = &mut doc.pages[frame.page_index];

                    for &sec_index in &frame.section_stack {
                        page.sections[sec_index].buf.push_str(line_str);
                        page.sections[sec_index].buf.push('\n');
                    }
                }
            }

            search_start = if line_end < src.len() {
                line_end + 1
            } else {
                src.len()
            };
        }

        if code_snapshot.is_some() {
            return Err(anyhow!("unterminated @code block"));
        }

        if !stack.is_empty() {
            return Err(anyhow!("unclosed page blocks"));
        }

        Ok(doc)
    }
}
//& @endpage
