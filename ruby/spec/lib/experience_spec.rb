require 'spec_helper'
require 'experience'

describe Experience do
  it 'has a current version' do
    v1 = Version.new challenge: ->(params) { params[:resurrect] }
    xp = Experience.new versions: [v1], params: {
      resurrect: true
    }

    expect(xp.current_version).to eq v1
  end

  it 'has a current level' do
    a_level = Level.new challenge: ->(params) { params[:a_level] }, name: 'a'
    b_level = Level.new challenge: ->(params) { params[:b_level] }, name: 'b'
    x_track = Track.new challenge: ->(params) { params[:x_track] },
                        levels: [a_level, b_level],
                        name: 'x'
    y_track = Track.new challenge: ->(params) { params[:y_track] },
                        levels: [a_level, b_level],
                        name: 'y'

    v1 = Version.new  tracks: [x_track, y_track],
                      challenge: ->(params) { params[:v1] }

    xp = Experience.new versions: [v1],
                        params: { x_track: true, a_level: true, v1: true }

    expect(xp.current_version).to eq v1
    expect(xp.current_track).to eq x_track
    expect(xp.current_level).to eq a_level

    xp = Experience.new versions: [v1],
                        params: { x_track: true, b_level: true, v1: true }
    expect(xp.current_version).to eq v1
    expect(xp.current_track).to eq x_track
    expect(xp.current_level).to eq b_level

    xp = Experience.new versions: [v1],
                        params: { y_track: true, a_level: true, v1: true }
    expect(xp.current_version).to eq v1
    expect(xp.current_track).to eq y_track
    expect(xp.current_level).to eq a_level

    xp = Experience.new versions: [v1], params: { v1: false }
    expect { xp.current_version }.to raise_error
  end
end
