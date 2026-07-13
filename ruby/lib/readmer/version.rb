# This is free and unencumbered software released into the public domain.

module Readmer; end
module Readmer::VERSION; end

module Readmer::VERSION
  FILE = File.expand_path('../../../VERSION', __FILE__)
  STRING = File.read(FILE).chomp.freeze
  MAJOR, MINOR, PATCH, EXTRA = STRING.split('.').map(&:freeze)
end # Readmer::VERSION
