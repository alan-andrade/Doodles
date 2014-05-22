require 'playable'
require 'version'
require 'track'
require 'level'

class Experience
  attr_reader :params

  def initialize(config)
    @versions = Playable::PSet.new(config.fetch(:versions, []))
    @params   = Playable::Params.new(config.fetch(:params))
  end

  def current_version
    @versions.play(@params)
  end

  def current_track
    current_version.tracks.play(@params)
  end

  def current_level
    current_track.levels.play(@params)
  end
end
