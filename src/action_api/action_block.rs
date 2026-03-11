use super::{ActionApiData, ActionApiRunnable, NoTitlesOrGenerator, Runnable};
use std::{collections::HashMap, marker::PhantomData};

type NoTarget = NoTitlesOrGenerator;

#[derive(Debug, Clone, Default)]
pub struct ActionApiBlockData {
    user: Option<String>,
    expiry: Option<String>,
    reason: Option<String>,
    anononly: bool,
    nocreate: bool,
    autoblock: bool,
    noemail: bool,
    hidename: bool,
    allowusertalk: bool,
    reblock: bool,
    newblock: bool,
    watchuser: bool,
    watchlistexpiry: Option<String>,
    tags: Option<Vec<String>>,
    partial: bool,
    pagerestrictions: Option<Vec<String>>,
    namespacerestrictions: Option<Vec<i64>>,
    actionrestrictions: Option<Vec<String>>,
    token: Option<String>,
}

impl ActionApiData for ActionApiBlockData {}

impl ActionApiBlockData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        params.insert("action".to_string(), "block".to_string());
        Self::add_str(&self.user, "user", &mut params);
        Self::add_str(&self.expiry, "expiry", &mut params);
        Self::add_str(&self.reason, "reason", &mut params);
        Self::add_boolean(self.anononly, "anononly", &mut params);
        Self::add_boolean(self.nocreate, "nocreate", &mut params);
        Self::add_boolean(self.autoblock, "autoblock", &mut params);
        Self::add_boolean(self.noemail, "noemail", &mut params);
        Self::add_boolean(self.hidename, "hidename", &mut params);
        Self::add_boolean(self.allowusertalk, "allowusertalk", &mut params);
        Self::add_boolean(self.reblock, "reblock", &mut params);
        Self::add_boolean(self.newblock, "newblock", &mut params);
        Self::add_boolean(self.watchuser, "watchuser", &mut params);
        Self::add_str(&self.watchlistexpiry, "watchlistexpiry", &mut params);
        Self::add_vec(&self.tags, "tags", &mut params);
        Self::add_boolean(self.partial, "partial", &mut params);
        Self::add_vec(&self.pagerestrictions, "pagerestrictions", &mut params);
        if let Some(ref v) = self.namespacerestrictions {
            let s: Vec<String> = v.iter().map(|n| n.to_string()).collect();
            params.insert("namespacerestrictions".to_string(), s.join("|"));
        }
        Self::add_vec(&self.actionrestrictions, "actionrestrictions", &mut params);
        Self::add_str(&self.token, "token", &mut params);
        params
    }
}

#[derive(Debug, Clone)]
pub struct ActionApiBlockBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiBlockData,
}

impl<T> ActionApiBlockBuilder<T> {
    pub fn expiry<S: AsRef<str>>(mut self, expiry: S) -> Self {
        self.data.expiry = Some(expiry.as_ref().to_string());
        self
    }

    pub fn reason<S: AsRef<str>>(mut self, reason: S) -> Self {
        self.data.reason = Some(reason.as_ref().to_string());
        self
    }

    pub fn anononly(mut self, anononly: bool) -> Self {
        self.data.anononly = anononly;
        self
    }

    pub fn nocreate(mut self, nocreate: bool) -> Self {
        self.data.nocreate = nocreate;
        self
    }

    pub fn autoblock(mut self, autoblock: bool) -> Self {
        self.data.autoblock = autoblock;
        self
    }

    pub fn noemail(mut self, noemail: bool) -> Self {
        self.data.noemail = noemail;
        self
    }

    pub fn hidename(mut self, hidename: bool) -> Self {
        self.data.hidename = hidename;
        self
    }

    pub fn allowusertalk(mut self, allowusertalk: bool) -> Self {
        self.data.allowusertalk = allowusertalk;
        self
    }

    pub fn reblock(mut self, reblock: bool) -> Self {
        self.data.reblock = reblock;
        self
    }

    pub fn newblock(mut self, newblock: bool) -> Self {
        self.data.newblock = newblock;
        self
    }

    pub fn watchuser(mut self, watchuser: bool) -> Self {
        self.data.watchuser = watchuser;
        self
    }

    pub fn watchlistexpiry<S: AsRef<str>>(mut self, watchlistexpiry: S) -> Self {
        self.data.watchlistexpiry = Some(watchlistexpiry.as_ref().to_string());
        self
    }

    pub fn tags<S: Into<String> + Clone>(mut self, tags: &[S]) -> Self {
        self.data.tags = Some(tags.iter().map(|s| s.clone().into()).collect());
        self
    }

    pub fn partial(mut self, partial: bool) -> Self {
        self.data.partial = partial;
        self
    }

    pub fn pagerestrictions<S: Into<String> + Clone>(mut self, pagerestrictions: &[S]) -> Self {
        self.data.pagerestrictions =
            Some(pagerestrictions.iter().map(|s| s.clone().into()).collect());
        self
    }

    pub fn namespacerestrictions(mut self, namespacerestrictions: &[i64]) -> Self {
        self.data.namespacerestrictions = Some(namespacerestrictions.to_vec());
        self
    }

    pub fn actionrestrictions<S: Into<String> + Clone>(
        mut self,
        actionrestrictions: &[S],
    ) -> Self {
        self.data.actionrestrictions =
            Some(actionrestrictions.iter().map(|s| s.clone().into()).collect());
        self
    }

    pub fn token<S: AsRef<str>>(mut self, token: S) -> Self {
        self.data.token = Some(token.as_ref().to_string());
        self
    }
}

impl ActionApiBlockBuilder<NoTarget> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiBlockData::default(),
        }
    }

    pub fn user<S: AsRef<str>>(mut self, user: S) -> ActionApiBlockBuilder<Runnable> {
        self.data.user = Some(user.as_ref().to_string());
        ActionApiBlockBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiRunnable for ActionApiBlockBuilder<Runnable> {
    fn params(&self) -> HashMap<String, String> {
        self.data.params()
    }

    fn http_method(&self) -> &'static str {
        "POST"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_builder() -> ActionApiBlockBuilder<NoTarget> {
        ActionApiBlockBuilder::new()
    }

    #[test]
    fn user_set() {
        let params = new_builder().user("SomeUser").data.params();
        assert_eq!(params["user"], "SomeUser");
    }

    #[test]
    fn expiry_set() {
        let params = new_builder().user("SomeUser").expiry("1 week").data.params();
        assert_eq!(params["expiry"], "1 week");
    }

    #[test]
    fn anononly_flag() {
        let params = new_builder().user("SomeUser").anononly(true).data.params();
        assert_eq!(params["anononly"], "");
    }

    #[test]
    fn anononly_false_absent() {
        let params = new_builder().user("SomeUser").data.params();
        assert!(!params.contains_key("anononly"));
    }

    #[test]
    fn token_set() {
        let params = new_builder()
            .user("SomeUser")
            .token("csrf+\\")
            .data
            .params();
        assert_eq!(params["token"], "csrf+\\");
    }

    #[test]
    fn action_is_block() {
        let params = new_builder().user("SomeUser").data.params();
        assert_eq!(params["action"], "block");
    }

    #[test]
    fn http_method_is_post() {
        let builder = new_builder().user("SomeUser");
        assert_eq!(builder.http_method(), "POST");
    }
}
