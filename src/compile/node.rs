#[derive(Debug, Clone, PartialEq, Default)]
pub struct Node {
    pub data: NodeData,
    pub branches: Vec<Node>,
    pub line: u32
}

#[allow(unused)]
#[derive(Debug, Clone, PartialEq, Default)]
pub enum NodeData {
    #[default]
    Root,

    // # Metadata
    SemVer, VM, Agent,

    // # Targets
    IsStage, CostumeNum, Layer, Volume,

    // # Targets (Stage)
    Tempo, VideoState, VideoTransparency, TTSLanguage,

    // # Targets (Sprite)
    Visible, XPos, YPos, Size, Direction, RotationStyle,

    // # Blocks
    Block, Uid, Opcode, Parent, Next, Input,
    Field, Mutation, Shadow, TopLevel, // XPos, YPos

    // # Costumes
    Name, Path, Format, BitmapRes, CenterX, CenterY,

    // # Sounds
    Rate, Samples, // Name, Path, Format

    // # Variables
    Variable, Value, IsCloud, // Name,

    // # Lists
    Item, // Name,

    // # Broadcasts
    // Uid, Name

    // # Monitors
    Mode, Param, SpriteName, Width, Height, SliderMin, SliderMax, IsDiscrete,
    // Opcode, Uid, Value, XPos, YPos, Visible

    // # Data
    PrototypeData(String), BlockPtrData(String), SubstackData(String),
    DoubleData(String), PosDoubleData(String), PosIntData(String),
    IntData(String), AngleData(String), ColorData(String),
    StringData(String), BroadcastData(String), VariableData(String),
    ListData(String), NullData
}