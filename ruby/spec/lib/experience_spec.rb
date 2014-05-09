require 'spec_helper'
require 'experience'

describe Experience do
  it 'has a current track' do
    x_track = Track.new challenge: ->(params) { params.key? :paid }, name: 'x'
    y_track = Track.new challenge: ->(params) { params[:resurrect] }, name: 'y'

    xp = Experience.new tracks: Playable::PSet.new([x_track, y_track]), params: {
      resurrect: true
    }

    expect(xp.current_track).to eq y_track
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

    xp = Experience.new tracks: [x_track, y_track],
                        params: { x_track: true, a_level: true }

    expect(xp.current_track).to eq x_track
    expect(xp.current_level).to eq a_level

    xp = Experience.new tracks: [x_track, y_track],
                        params: { x_track: true, b_level: true }
    expect(xp.current_track).to eq x_track
    expect(xp.current_level).to eq b_level

    xp = Experience.new tracks: [x_track, y_track],
                        params: { y_track: true, a_level: true }
    expect(xp.current_track).to eq y_track
    expect(xp.current_level).to eq a_level
  end
end
