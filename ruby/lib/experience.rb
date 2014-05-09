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

class Track
  include Playable

  attr_reader :name

  def initialize config
    @levels = Playable::PSet.new(config.fetch(:levels, []))
    @name = config.fetch(:name)
    super
  end

  def current_level(params)
    @levels.play(params)
  end
end

class Level
  include Playable

  attr_reader :name

  def initialize config
    @name = config.fetch(:name)
    super
  end
end
