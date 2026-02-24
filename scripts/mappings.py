#=================================================================================#
#                                                                                 #
#    docWX: A minimal, performant book-keeping, authoring and documentation tool. #
#    Copyright (C) 2026  argmaxin                                                 #
#                                                                                 #
#    This program is free software: you can redistribute it and/or modify         #
#    it under the terms of the GNU General Public License as published by         #
#    the Free Software Foundation, either version 3 of the License, or            #
#    (at your option) any later version.                                          #
#                                                                                 #
#    This program is distributed in the hope that it will be useful,              #
#    but WITHOUT ANY WARRANTY; without even the implied warranty of               #
#    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the                #
#    GNU General Public License for more details.                                 #
#                                                                                 #
#    You should have received a copy of the GNU General Public License            #
#    along with this program.  If not, see <https://www.gnu.org/licenses/>.       #
#                                                                                 #
#=================================================================================#

LANG_COMM_PREFIXES = {
    # Extensions
    ".bazel": "#&",
    ".c":    "//&",
    ".h":    "//&",
    ".cpp":  "//&",
    ".cc":   "//&",
    ".cxx":  "//&",
    ".hpp":  "//&",
    ".hh":   "//&",
    ".hxx":  "//&",
    ".m":    "//&",
    ".mm":   "//&",
    ".cu":   "//&",
    ".cuh":  "//&",
    ".rs":   "//&",
    ".zig":  "//&",
    ".d":    "//&",
    ".java": "//&",
    ".cs":   "//&",
    ".kt":   "//&",
    ".kts":  "//&",
    ".scala":"//&",
    ".swift":"//&",
    ".go":   "//&",
    ".v":    "//&",
    ".jai":  "//&",
    ".py":   "#&",
    ".sh":   "#&",
    ".bash": "#&",
    ".zsh":  "#&",
    ".fish": "#&",
    ".rb":   "#&",
    ".pl":   "#&",
    ".ps1":  "#&",
    ".pm":   "#&",
    ".tcl":  "#&",
    ".tf":   "#&",
    ".awk":  "#&",
    ".nim":  "#&",
    ".jl":   "#&",
    ".hs":   "--&",
    ".elm":  "--&",
    ".ml":   "*&",
    ".mli":  "*&",
    ".fs":   "//&",
    ".fsi":  "//&",
    ".R":    "#&",
    ".re":   "//&",
    ".rei":  "//&",
    ".lisp": ";;&",
    ".cl":   ";;&", 
    ".scm":  ";;&",
    ".ss":   ";;&",
    ".rkt":  ";;&",
    ".clj":  ";;&",
    ".cljs": ";;&",
    ".cljc": ";;&",
    ".el":   ";;&",
    ".js":   "//&",
    ".mjs":  "//&",
    ".cjs":  "//&",
    ".ts":   "//&",
    ".tsx":  "//&",
    ".jsx":  "//&",
    ".mk":     "#&",
    ".cmake": "#&",
    ".toml":  "#&",
    ".ini":   "#&",
    ".conf":  "#&",
    ".yml":   "#&",
    ".yaml":  "#&",
    ".bzl":   "#&",
    ".s":    ";&",
    ".asm":  ";&",
    
    "Makefile":        "#&",
    "GNUMakefile":     "#&",
    "BSDmakefile":     "#&",
    "CMakeLists.txt":  "#&",
    "meson.build":    "#&",
    "meson_options.txt": "#&",
    "BUILD":           "#&",
    "WORKSPACE":      "#&",
    "Dockerfile":     "#&",
    ".gitignore":     "#&",
    ".gitmodules":    "#&",
    ".gitattributes": "#&",
    ".gitlab-ci.yml": "#&",
    ".gitlab-ci.yaml":"#&",
    ".bashrc":        "#&",
    ".bash_profile": "#&",
    ".profile":      "#&",
    ".zshrc":        "#&",
    ".zprofile":     "#&",
    ".zshenv":       "#&",
    ".editorconfig": "#&",
    ".env":           "#&",
    "requirements.txt": "#&",
    "pip.conf":        "#&",
}

def get_maps():
    return LANG_COMM_PREFIXES
