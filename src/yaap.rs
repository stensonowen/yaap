
pub trait BuilderState {}

#[derive(Debug)] #[must_use] pub struct YaapOpts;
#[derive(Debug)] #[must_use] pub struct YaapArgs;
#[derive(Debug)] pub struct YaapDone;

impl BuilderState for YaapOpts {}
impl BuilderState for YaapArgs {}
impl BuilderState for YaapDone {}



