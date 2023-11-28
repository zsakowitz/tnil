use vec1::Vec1;

use crate::{
    category::{AppositiveCase, Referent, ReferentialAffixPerspective, ThematicCase},
    gloss::{Gloss, GlossFlags, GlossStatic},
};

/// A referential affix. The `CaseType` generic specifies the type of case this affix uses, to allow
/// this type to be reused between thematic and appositive referential affixes.
///
/// Two common variants of this type, based on thematic and appositive cases, are exposed under the
/// aliases [`ThematicReferentialAffix`] and [`AppositiveReferentialAffix`] respectively.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ReferentialAffix<CaseType> {
    /// The referents of this affix.
    pub referents: Vec1<Referent>,

    /// The perspective of this affix.
    pub perspective: ReferentialAffixPerspective,

    /// The case of this affix.
    pub case: CaseType,
}

impl<CaseType> ReferentialAffix<CaseType> {
    /// Creates a new [`ReferentialAffix`].
    pub fn new(
        referents: Vec1<Referent>,
        perspective: ReferentialAffixPerspective,
        case: CaseType,
    ) -> Self {
        Self {
            referents,
            perspective,
            case,
        }
    }
}

/// A referential affix containing a [`ThematicCase`].
pub type ThematicReferentialAffix = ReferentialAffix<ThematicCase>;

/// A referential affix containing an [`AppositiveCase`].
pub type AppositiveReferentialAffix = ReferentialAffix<AppositiveCase>;

impl<CaseType> Gloss for ReferentialAffix<CaseType>
where
    CaseType: Copy + GlossStatic,
{
    fn gloss(&self, flags: GlossFlags) -> String {
        let needs_brackets = self.referents.len() != 0
            || self.perspective != ReferentialAffixPerspective::M
            || flags.matches(GlossFlags::SHOW_DEFAULTS);

        let mut output = "(".to_owned();

        if needs_brackets {
            output += "[";
        }

        let mut is_first_segment = true;

        for referent in &self.referents {
            if !is_first_segment {
                output += "+";
            }

            is_first_segment = false;

            output += &referent.gloss(flags);
        }

        if self.perspective != ReferentialAffixPerspective::M
            || flags.matches(GlossFlags::SHOW_DEFAULTS)
        {
            if !is_first_segment {
                output += "+";
            }

            output += self.perspective.gloss_static(flags);
        }

        if needs_brackets {
            output += "]";
        }

        output += "-";
        output += self.case.gloss_static(flags);
        output += ")";
        output
    }
}
