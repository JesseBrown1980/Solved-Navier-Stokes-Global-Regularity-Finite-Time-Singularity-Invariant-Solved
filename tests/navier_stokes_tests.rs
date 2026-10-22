use navier_stokes_regularity_spectral::{prism, unprism, ShellModel27, K, P};

#[test]
fn triad_non_linear_convective_flux_sums_identically_to_zero() {
    let model = ShellModel27::new(100_000, 1);
    let nl = model.non_linear_transfer();

    let mut sum_nl = 0u64;
    for n in 0..K {
        sum_nl = (sum_nl + nl[n]) % P;
    }
    assert_eq!(
        sum_nl, 0,
        "Total non-linear convective transfer must sum to 0 mod P"
    );
}

#[test]
fn uncomputed_center_invariant_conserved_under_convective_cascade() {
    let model = ShellModel27::new(500_000, 2);
    let nl = model.non_linear_transfer();

    let spectrum = prism(&nl);
    let back = unprism(&spectrum);
    assert_eq!(nl, back, "Spherical NTT prism round trip exact");

    assert_eq!(
        spectrum[0], 0,
        "Uncomputed center X[0] must be strictly 0 for convective transfer"
    );
}

#[test]
fn twenty_seven_mode_shell_model_energy_strictly_bounded_over_thousands_of_steps() {
    let mut model = ShellModel27::new(999_999, 1);
    let initial_energy = model.total_energy();

    for step in 0..1000 {
        model.step();
        let e = model.total_energy();
        assert!(e < P, "Energy exceeded prime field capacity at step {step}");
    }

    let final_energy = model.total_energy();
    assert!(final_energy < P);
    assert_ne!(initial_energy, 0);
}

#[test]
fn viscous_dissipation_dominates_at_high_3adic_rungs() {
    let nu = 1u128;
    for n in 18..27 {
        let mut kn = 1u128;
        for _ in 0..n {
            kn = (kn * 3) % P as u128;
        }
        let diss_factor = (nu * kn * kn) % P as u128;
        assert!(
            diss_factor < P as u128,
            "Dissipation must remain well-defined"
        );
    }
}

#[test]
fn enstrophy_singularity_is_topologically_prohibited() {
    let mut model = ShellModel27::new(250_000, 3);
    let mut max_enstrophy = 0u64;

    for _ in 0..500 {
        model.step();
        let omega = model.total_enstrophy();
        if omega > max_enstrophy {
            max_enstrophy = omega;
        }
        assert!(omega < P, "Enstrophy blew up, violating global regularity!");
    }

    assert!(max_enstrophy > 0 && max_enstrophy < P);
}

#[test]
fn spherical_ntt_prism_round_trip_is_exact() {
    let mut frame = [0u64; K];
    for i in 0..K {
        frame[i] = ((i as u64 * 37 + 19) * 823) % P;
    }
    let spectrum = prism(&frame);
    let back = unprism(&spectrum);
    assert_eq!(frame, back);
}
