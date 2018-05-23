use audio::gain_node::GainNodeOptions;
use audio::oscillator_node::OscillatorNodeOptions;
use audio::block::Chunk;

pub enum AudioNodeType {
    AnalyserNode,
    BiquadFilterNode,
    AudioBuffer,
    AudioBufferSourceNode,
    ChannelMergerNode,
    ChannelSplitterNode,
    ConstantSourceNode,
    ConvolverNode,
    DelayNode,
    DestinationNode,
    DynamicsCompressionNode,
    GainNode(GainNodeOptions),
    IIRFilterNode,
    OscillatorNode(OscillatorNodeOptions),
    PannerNode,
    PeriodicWave,
    ScriptProcessorNode,
    StereoPannerNode,
    WaveShaperNode,
}

pub trait AudioNodeEngine: Send {
    // XXX Create an AudioBuffer abstraction
    fn process(
        &self,
        inputs: Chunk,
        rate: u32,
    ) -> Chunk;
}