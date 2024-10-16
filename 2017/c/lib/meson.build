libaoc_include = include_directories('.')
libaoc = library('libaoc',
  sources: ['aoc.c', 'aoc.h'])

libaoc_dep = declare_dependency(
  include_directories: [libaoc_include],
  link_with: libaoc)
