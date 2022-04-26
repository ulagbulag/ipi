#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct GuarantorSigned<T> {
    pub guarantor: Identity,
    pub data: GuaranteeSigned<T>,
}

impl<T> ::core::ops::Deref for GuarantorSigned<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> ::core::ops::DerefMut for GuarantorSigned<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct GuaranteeSigned<T> {
    pub guarantee: Identity,
    pub data: T,
}

impl<T> ::core::ops::Deref for GuaranteeSigned<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> ::core::ops::DerefMut for GuaranteeSigned<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

#[derive(Clone, Debug, Eq, Serialize, Deserialize)]
pub struct Identity {
    pub public_key: ed25519_dalek::PublicKey,
    pub signature: ed25519_dalek::Signature,
}

impl PartialEq for Identity {
    fn eq(&self, other: &Self) -> bool {
        self.public_key == other.public_key && self.signature == other.signature
    }
}

impl PartialOrd for Identity {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self
            .public_key
            .as_ref()
            .partial_cmp(other.public_key.as_ref())
        {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.signature
            .as_ref()
            .partial_cmp(other.signature.as_ref())
    }
}

impl Ord for Identity {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.public_key
            .as_ref()
            .cmp(other.public_key.as_ref())
            .then(self.signature.as_ref().cmp(other.signature.as_ref()))
    }
}

impl ::core::hash::Hash for Identity {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.public_key.as_ref().hash(state);
        self.signature.as_ref().hash(state);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub private_key: ed25519_dalek::SecretKey,
}
