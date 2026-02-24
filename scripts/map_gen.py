#=================================================================================#
#                                                                                 #
#    docWX: A minimal, performant book-keeping, authoring and documentation tool. #
#    Copyright (C) 2026  Xe64                                                     #
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

import mappings

def enum_name(key):
    name = key.upper()
    name = name.replace(".", "")
    name = name.replace("-", "_")
    return f"LA_{name}"

def generate_header(lang_map):
    keys = sorted(lang_map.keys(), reverse=True)

    with open("table.rs", "w") as f:
        f.write("/*  docWX: A minimal, performant book-keeping, authoring and documentation tool.\n")
        f.write("\tCopyright (C) 2026  Xe64\n")
        f.write("\t\n")
        f.write("\tThis program is free software: you can redistribute it and/or modify\n")
        f.write("\tit under the terms of the GNU General Public License as published by\n")
        f.write("\tthe Free Software Foundation, either version 3 of the License, or\n")
        f.write("\t(at your option) any later version.\n")
        f.write("\t\n")
        f.write("\tThis program is distributed in the hope that it will be useful,\n")
        f.write("\tbut WITHOUT ANY WARRANTY; without even the implied warranty of\n")
        f.write("\tMERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the\n")
        f.write("\tGNU General Public License for more details.\n")
        f.write("\t\n")
        f.write("\tYou should have received a copy of the GNU General Public License\n")
        f.write("\talong with this program.  If not, see <https://www.gnu.org/licenses/>.*/\n\n")
        f.write("/* auto-generated, do not edit */\n")
        f.write("\n")
        f.write("use phf::phf_map;\n\n")
        f.write("pub const fn build_tables() -> phf::Map<&'static str, &'static str> {\n")
        f.write("\tphf_map!{\n\n")
        for k in keys:
            f.write(f"\t \"{k}\" => \"{lang_map[k]}\",\n")
        f.write("}\n")
        f.write("}\n")
        f.write("\n")

if __name__ == "__main__":
    generate_header(mappings.get_maps())
