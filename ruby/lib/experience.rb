require 'playable'

class Experience
  attr_reader :params

  def initialize(config)
    @tracks = Playable::PSet.new(config.fetch(:tracks, []))
    @params = Playable::Params.new(config.fetch(:params))
  end

  def current_track
    @tracks.play(@params)
  end

  def current_level
    current_track.current_level(@params)
  end
end
