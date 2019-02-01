#!/usr/bin/env ruby

filepath = 'src/spec.rs'
text = File.read(filepath)

# Horrifying hack to add Default to structs, except the ones containing enums with no default.
re = /Deserialize(\)\]\n)([\w ]+struct (?!(NamespaceReference|Seccomp|Syscall|SyscallArg|Image|WindowsDevice|StateSchema))[\w]+ \{)/m
text = text.gsub(re, "Deserialize, Clone, Default, Builder, PartialEq\\1#[builder(setter(into))]\n\\2")

# Add PartialEq to everything.
re = /Deserialize(\)\]\n)([\w ]+ struct [\w]+ \{)/m
text = text.gsub(re, "Deserialize, Clone, Builder, PartialEq\\1#[builder(setter(into))]\n\\2")

# Add PartialEq to everything.
re = /Deserialize(\)\]\n[\w ]+enum [\w]+ \{)/m
text = text.gsub(re, 'Deserialize, Clone, PartialEq\1')

# Add PartialEq to everything.
re = /(^\s+pub [\w]+: (?:(Option)).+,$)/
text = text.gsub(re, "#[builder(default)]\n\\1")

File.open(filepath, "w") {|file| file.puts(text)}
