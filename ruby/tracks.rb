#XXX: giak, terrible name
module Matchable
  def initialize config
    # Rule is a block that will be evaluated passing the "params".
    #
    # You put logic here that if returns a truthy result, will "match" and
    # pick that entity (level/track)
    @rule = config.fetch(:rule)
  end

  def match? params
    params.check!
    @rule.call(params)
  end
end

class Level
  include Matchable

  attr_reader :name

  def initialize config
    @name = config.fetch(:name)
    super # Call matchable constructor
  end
end

class Track
  include Matchable

  attr_reader :name

  def initialize config
    @levels = config.fetch(:levels, []).to_set
    @name = config.fetch(:name)
    super
    # @rule more expressive if its enforced to be per class, not per instance.
    # @rule = config.fetch(:rule)
  end

  def current_level(params)
    @levels.find { |level| level.match?(params) }
  end
end

class Experience
  def initialize(config)
    @tracks = config.fetch(:tracks, []).to_set
    @params = Params.new
    @params.merge! config.fetch(:params)
  end

  def current_track
    @tracks.find { |track| track.match?(@params) } || raise('No track found')
  end

  def current_level
    current_track.current_level(@params)
  end
end

class Params < Hash
  def check!
    # probably check that user exist or any other key-value exists.
  end
end

describe Experience do
  it 'has a current track' do
    x_track = Track.new rule: ->(params) { params.key? :paid }, name: 'x'
    y_track = Track.new rule: ->(params) { params[:resurrect] }, name: 'y'

    xp = Experience.new tracks: [x_track, y_track], params: {
      resurrect: true
    }

    expect(xp.current_track).to eq y_track
  end

  it 'has a current level' do
    a_level = Level.new rule: ->(params) { return params[:a_level] }, name: 'a'
    b_level = Level.new rule: ->(params) { return params[:b_level] }, name: 'b'
    x_track = Track.new rule: ->(params) { return params[:x_track] },
                        levels: [a_level, b_level],
                        name: 'x'
    y_track = Track.new rule: ->(params) { params[:y_track] },
                        levels: [a_level, b_level],
                        name: 'y'

    xp = Experience.new tracks: [x_track, y_track], params: { x_track: true,
                                                              a_level: true
                                                             }
    expect(xp.current_track).to eq x_track
    expect(xp.current_level).to eq a_level

    xp = Experience.new tracks: [x_track, y_track], params: { x_track: true,
                                                              b_level: true
                                                            }
    expect(xp.current_track).to eq x_track
    expect(xp.current_level).to eq b_level

    xp = Experience.new tracks: [x_track, y_track], params: { y_track: true,
                                                              a_level: true
                                                            }
    expect(xp.current_track).to eq y_track
    expect(xp.current_level).to eq a_level
  end
end
