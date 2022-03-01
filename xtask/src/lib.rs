//! `cargo xtask` automation.
//!
//! Please refer to <https://github.com/matklad/cargo-xtask/> for an explanation of the concept.

use std::process::Command;

pub fn install_targets(targets: &mut dyn Iterator<Item = &str>, toolchain: Option<&str>) {
    let mut rustup = Command::new("rustup");
    rustup.arg("target").arg("add").args(targets);

    if let Some(toolchain) = toolchain {
        rustup.arg("--toolchain").arg(toolchain);
    }

    let status = rustup.status().unwrap();
    assert!(status.success(), "rustup command failed: {:?}", rustup);
}

// Check that serde and PartialOrd works with VectActive
pub fn check_host_side() {
    use cortex_m::peripheral::{itm::LocalTimestampOptions, scb::VectActive};

    // check serde
    {
        let v = VectActive::from(22).unwrap();
        let json = serde_json::to_string(&v).expect("Failed to serialize VectActive");
        let deser_v: VectActive =
            serde_json::from_str(&json).expect("Failed to deserialize VectActive");
        assert_eq!(deser_v, v);

        let lts = LocalTimestampOptions::EnabledDiv4;
        let json = serde_json::to_string(&lts).expect("Failed to serialize LocalTimestampOptions");
        let deser_lts: LocalTimestampOptions =
            serde_json::from_str(&json).expect("Failed to deserilaize LocalTimestampOptions");
        assert_eq!(deser_lts, lts);
    }

    // check PartialOrd
    {
        let a = VectActive::from(19).unwrap();
        let b = VectActive::from(20).unwrap();
        assert!(a < b);
    }

    // check TryFrom
    {
        use core::convert::TryInto;
        use std::convert::TryFrom;

        let lts: LocalTimestampOptions = (16_u8).try_into().unwrap();
        assert_eq!(lts, LocalTimestampOptions::EnabledDiv16);

        assert!(LocalTimestampOptions::try_from(42).is_err());
    }
}
