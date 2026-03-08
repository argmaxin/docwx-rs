//& @page("README")
//&  @section(0)
//& # docWX
//& 
//& ## About
//& 
//& A minimal, performant book-keeping, authoring and documentation tool.
//& Extracts structured documentation from source comments and writes Markdown pages.
//& Licensed under GNU GPL-v3-or-later.
//&
//& ## Usage
//&
//& ### crates.io (Recommended)
//& 
//& Run: `cargo install docwx`
//&
//& ### Local
//& Build: `cargo build --release`.
//& Install (if needed): `cargo install --path .`
//& 
//& Example: 
//& `docwx -i src -i include -o spec -g`
//&
//& What the each of the flags do:
//& - `-i` \ `--input`: provide source(s).
//& - `-o` \ `--output`: provide output directory.
//& - `-g` \ `--gitignore`: if used, respect .gitignore/.gitinfo/.gitexclude.
//& - `-d` \ `--depth`: specify the maximum filesystem depth.
//& 
//& ## Why? 
//& 
//& For documentation, duh. 
//& Jokes aside, it's because documentation is tedious. 
//& 
//& Now there exists standard, good quality and battle-tested tools, however I found that **none** of them did what I **wanted**.
//& 
//& Namely, I wished to have a **tool** that: 
//& - allowed me to write documentation **where** the code is **AND**
//& - **across** and **interwoven** between files **AND**
//& - without needing knowledge regarding the language (if any) **AND**
//& - is sufficiently performant to **handle** my codebase complexity **AND**
//& - not actively get in my **way** by requiring obscure configurations or complex DSLs I needed to learn **AND** 
//& - output it in Markdown format.
//& 
//& Which sharply decreased my options, and hence **docWX**.
//& 
//& ## How?
//& 
//& docWX scans source files for a comment-prefixed DSL.
//& 
//& Everything after the configured prefix is treated as docWX input.
//& The host language is irrelevant.
//& 
//& Example (C-style comments):
//& ```c
//& //& @page("intro")
//& //&  @section(10)
//& //&    # Index
//& //&    This appears first.
//& //&  @endsection
//& //&  @section(100)
//& //&    ## Hello
//& //&   Welcome!
//& //&  @endsection
//& //& @endpage
//& ```
//& 
//& docWX:
//&
//& - Walks directories recursively
//& - Detects files by extension or name
//& - Extracts DSL blocks from comments
//& - Orders sections numerically (per page)
//& - Writes the result as Markdown
//&
//& I could naturally reverse the order of `@section`:
//& ```c
//& //& @page("intro")
//& //&  @section(100)
//& //&    ## Hello
//& //&    Welcome!
//& //&  @endsection
//& //&  @section(10)
//& //&    # Index
//& //&    This appears first.
//& //&  @endsection
//& //& @endpage
//& ```
//& 
//& And the same for python:
//& ```py
//& #& @page("intro")
//& #&   @section(100)
//& #&     ## Hello
//& #&     Welcome!
//& #&   @endsection
//& #&   @section(10)
//& #&     # Index
//& #&     This appears first.
//& #&   @endsection
//& #& @endpage
//& ```
//& 
//& What matters is the **numeric value** inside `@section(N)`, which docWX uses to order sections within a page. 
//& 
//& More of that later.
//& 
//& ## What?
//& 
//& Before you get your expectations high, it is natural to take a look at what docWX might offer and whether that is acceptable to you.
//& 
//& ### Features
//& 
//& docWX ensures the following:
//& - Comment-based DSL
//& - Language-agnostic input
//& - Deterministic output
//& - Per-page numeric section ordering
//& - Section projection into multiple pages
//& - Coexists with tools like Doxygen, rustdoc, etc.
//& - No runtime configuration
//& - No plugins
//& - No editor lock-in
//& 
//& ### Non-features
//& 
//& docWX does not:
//& - parse programming languages
//& - understand symbols
//& - track functions or types
//& - auto-generate API references
//& - guess intent
//& - reorder content heuristically
//& - provide anchors into code
//& - validate correctness beyond its own grammar
//& 
//& If you want that, use your compiler or existing doc tools. docwx is compatible and can be used alongside them.
//& 
//& ## Grammar
//& 
//& The DSL is kept minimal, and stupid too. 
//& Inspirations taken from the C-preprocessor.
//& 
//& ### Directives
//& 
//& - `@page("path")`: Declares a Markdown output page.
//& - `@section(N)`: Declares a section with numeric ID N.
//& - `@endpage` and `@endsection`: Mark the ends of the above two.
//& - `@code` and `@endcode`: To include raw code. Helps avoid copy-pasting.
//&
//& ### Rules
//& 
//& - Directives **must appear** on their own line
//& - Section numbers are:
//&     - required
//&     - numeric
//&     - unique **per page**
//& - Ordering is **strictly numeric**
//& - Duplicate section IDs are errors
//& - Nesting creates `projection`, not hierarchy
//& 
//& If you ask me what this so-called `projection` might be, consider the following Rust code:
//& ```rust
//& //& @page("important")
//& //& @section(100) 
//& //& # Important Talk
//& //& This is something very important.
//& //& We want to include this section.
//& //& @code
//& const fn very_important_function(n: u32) -> u32 {
//&    n * n
//& }
//& //& @endcode
//& //&  @page("other") 
//& //&   @section(50) 
//& //& Appears in both places.
//& //&   @endsection
//& //&  @endpage
//& //& @endsection
//& //& @endpage
//& ```
//& 
//& The text is written once and emitted in multiple pages.
//& This allows documentation written in one place to appear in multiple outputs without duplication.
//& 
//& That's all you need to know (or should care to know).
//& 
//& ## Contribution
//& 
//& docWX is intentionally small.
//& 
//& Before proposing changes, ask:
//& - Does this add new semantics?
//& - Does this require language awareness?
//& - Does this make the tool smarter instead of clearer?
//& 
//& If the answer is yes, the change will likely be rejected.
//& 
//& Acceptable contributions:
//& - bug fixes
//& - documentation improvements
//& - additional file prefix mappings
//& - portability fixes
//& - clearer error messages
//& 
//& docWX values:
//& - predictability over flexibility
//& - explicit rules over heuristics
//& - correctness (even if painful) over cleverness
//& 
//& ## Conclusion
//& 
//& Give docWX a try, perhaps? Also save your complaints about verbosity, because I did too. 
//& 
//& If you have read so far (or even skimmed/scrolled down), thank you for your time and consideration. 
//& I hope docWX is to your liking (except the name maybe, I tried, sorry).
//& 
//&  @endsection
//& @endpage
