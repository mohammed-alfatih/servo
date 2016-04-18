/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::CSSRuleListBinding;
use dom::bindings::codegen::Bindings::CSSRuleListBinding::CSSRuleListMethods;
use dom::bindings::global::GlobalRef;
use dom::bindings::js::{JS, Root};
use dom::bindings::reflector::{Reflector, reflect_dom_object};
use dom::cssrule::CSSRule;
use dom::cssstylesheet::CSSStyleSheet;
use dom::window::Window;

#[dom_struct]
pub struct CSSRuleList {
    reflector_: Reflector,
    cssrule: JS<CSSRule>,
}

impl CSSRuleList {
    #[allow(unrooted_must_root)]
    fn new_inherited(rule: JS<CSSRule>) -> CSSRuleList {
        CSSRuleList {
            reflector_: Reflector::new(),
            cssrule: rule
        }
    }

    #[allow(unrooted_must_root)]
    pub fn new(window: &Window, cssrule: JS<CSSRule>) -> Root<CSSRuleList> {
        reflect_dom_object(box CSSRuleList::new_inherited(cssrule),
                           GlobalRef::Window(window), CSSRuleListBinding::Wrap)
    }
}

impl CSSRuleListMethods for CSSRuleList {
    // https://drafts.csswg.org/cssom/#dom-stylesheetlist-length
    fn Length(&self) -> u32 {
       self.CSSStyleSheet.cssrules().len() as u32
    }

    // https://drafts.csswg.org/cssom/#dom-stylesheetlist-item
    fn Item(&self, index: u32) -> Option<Root<CSSRule>> {
        None
        //TODO Create a new StyleSheet object and return it
    }

    // check-tidy: no specs after this line
    fn IndexedGetter(&self, index: u32, found: &mut bool) -> Option<Root<CSSRule>>{
        let item = self.Item(index);
        *found = item.is_some();
        item
    }
}
