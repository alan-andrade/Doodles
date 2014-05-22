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

