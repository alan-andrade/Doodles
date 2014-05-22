class Track
  include Playable

  attr_reader :name, :levels

  def initialize config
    @levels = Playable::PSet.new(config.fetch(:levels, []))
    @name = config.fetch(:name)
    super
  end
end

