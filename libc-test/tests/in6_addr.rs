//! Compare libc's IN6_IS_ADDR_* family of functions against the actual C macros,
//! for a range of interesting addresses.

#[cfg(any(
    all(target_os = "linux", not(target_arch = "wasm32")),
    target_os = "android",
    target_os = "emscripten",
    target_vendor = "apple",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd",
    target_os = "solaris",
    target_os = "illumos",
))]
mod t {
    use libc::{
        self,
        in6_addr,
    };

    type Predicate = extern "C" fn(&in6_addr) -> bool;
    type CPredicate = unsafe extern "C" fn(*const in6_addr) -> bool;

    extern "C" {
        fn in6_is_addr_unspecified(a: *const in6_addr) -> bool;
        fn in6_is_addr_loopback(a: *const in6_addr) -> bool;
        fn in6_is_addr_multicast(a: *const in6_addr) -> bool;
        fn in6_is_addr_linklocal(a: *const in6_addr) -> bool;
        fn in6_is_addr_sitelocal(a: *const in6_addr) -> bool;
        fn in6_is_addr_v4mapped(a: *const in6_addr) -> bool;
        fn in6_is_addr_v4compat(a: *const in6_addr) -> bool;
        fn in6_is_addr_mc_nodelocal(a: *const in6_addr) -> bool;
        fn in6_is_addr_mc_linklocal(a: *const in6_addr) -> bool;
        fn in6_is_addr_mc_sitelocal(a: *const in6_addr) -> bool;
        fn in6_is_addr_mc_orglocal(a: *const in6_addr) -> bool;
        fn in6_is_addr_mc_global(a: *const in6_addr) -> bool;
        fn in6_are_addr_equal(a: *const in6_addr, b: *const in6_addr) -> bool;

        #[cfg(target_os = "android")]
        fn in6_is_addr_ula(a: *const in6_addr) -> bool;
        #[cfg(target_os = "android")]
        fn ipv6_addr_mc_scope(a: *const in6_addr) -> u8;

        #[cfg(target_vendor = "apple")]
        fn in6_is_addr_unique_local(a: *const in6_addr) -> bool;
        #[cfg(target_vendor = "apple")]
        fn in6_is_addr_unicast_based_multicast(a: *const in6_addr) -> bool;
        #[cfg(target_vendor = "apple")]
        fn ipv6_addr_mc_flags(a: *const in6_addr) -> u8;

        #[cfg(target_os = "openbsd")]
        fn in6_is_addr_mc_intfacelocal(a: *const in6_addr) -> bool;

        #[cfg(target_os = "illumos")]
        fn in6_is_addr_v4mapped_any(a: *const in6_addr) -> bool;
        #[cfg(target_os = "illumos")]
        fn in6_is_addr_mc_reserved(a: *const in6_addr) -> bool;
        #[cfg(target_os = "illumos")]
        fn in6_is_addr_mc_solicitednode(a: *const in6_addr) -> bool;
        #[cfg(target_os = "illumos")]
        fn in6_is_addr_6to4(a: *const in6_addr) -> bool;
        #[cfg(target_os = "illumos")]
        fn in6_is_addr_linkscope(a: *const in6_addr) -> bool;
        #[cfg(target_os = "illumos")]
        fn in6_are_6to4_prefix_equal(a: *const in6_addr, b: *const in6_addr) -> bool;
    }

    fn predicates() -> Vec<(&'static str, Predicate, CPredicate)> {
        // Only some platforms add their extensions
        #[allow(unused_mut)]
        let mut predicates: Vec<(&'static str, Predicate, CPredicate)> = vec![
            (
                "IN6_IS_ADDR_UNSPECIFIED",
                libc::IN6_IS_ADDR_UNSPECIFIED,
                in6_is_addr_unspecified,
            ),
            (
                "IN6_IS_ADDR_LOOPBACK",
                libc::IN6_IS_ADDR_LOOPBACK,
                in6_is_addr_loopback,
            ),
            (
                "IN6_IS_ADDR_MULTICAST",
                libc::IN6_IS_ADDR_MULTICAST,
                in6_is_addr_multicast,
            ),
            (
                "IN6_IS_ADDR_LINKLOCAL",
                libc::IN6_IS_ADDR_LINKLOCAL,
                in6_is_addr_linklocal,
            ),
            (
                "IN6_IS_ADDR_SITELOCAL",
                libc::IN6_IS_ADDR_SITELOCAL,
                in6_is_addr_sitelocal,
            ),
            (
                "IN6_IS_ADDR_V4MAPPED",
                libc::IN6_IS_ADDR_V4MAPPED,
                in6_is_addr_v4mapped,
            ),
            (
                "IN6_IS_ADDR_V4COMPAT",
                libc::IN6_IS_ADDR_V4COMPAT,
                in6_is_addr_v4compat,
            ),
            (
                "IN6_IS_ADDR_MC_NODELOCAL",
                libc::IN6_IS_ADDR_MC_NODELOCAL,
                in6_is_addr_mc_nodelocal,
            ),
            (
                "IN6_IS_ADDR_MC_LINKLOCAL",
                libc::IN6_IS_ADDR_MC_LINKLOCAL,
                in6_is_addr_mc_linklocal,
            ),
            (
                "IN6_IS_ADDR_MC_SITELOCAL",
                libc::IN6_IS_ADDR_MC_SITELOCAL,
                in6_is_addr_mc_sitelocal,
            ),
            (
                "IN6_IS_ADDR_MC_ORGLOCAL",
                libc::IN6_IS_ADDR_MC_ORGLOCAL,
                in6_is_addr_mc_orglocal,
            ),
            (
                "IN6_IS_ADDR_MC_GLOBAL",
                libc::IN6_IS_ADDR_MC_GLOBAL,
                in6_is_addr_mc_global,
            ),
        ];

        #[cfg(target_os = "android")]
        predicates.push(("IN6_IS_ADDR_ULA", libc::IN6_IS_ADDR_ULA, in6_is_addr_ula));

        #[cfg(target_vendor = "apple")]
        predicates.extend([
            (
                "IN6_IS_ADDR_UNIQUE_LOCAL",
                libc::IN6_IS_ADDR_UNIQUE_LOCAL as Predicate,
                in6_is_addr_unique_local as CPredicate,
            ),
            (
                "IN6_IS_ADDR_UNICAST_BASED_MULTICAST",
                libc::IN6_IS_ADDR_UNICAST_BASED_MULTICAST,
                in6_is_addr_unicast_based_multicast,
            ),
        ]);

        #[cfg(target_os = "openbsd")]
        predicates.push((
            "IN6_IS_ADDR_MC_INTFACELOCAL",
            libc::IN6_IS_ADDR_MC_INTFACELOCAL,
            in6_is_addr_mc_intfacelocal,
        ));

        #[cfg(target_os = "illumos")]
        predicates.extend([
            (
                "IN6_IS_ADDR_V4MAPPED_ANY",
                libc::IN6_IS_ADDR_V4MAPPED_ANY as Predicate,
                in6_is_addr_v4mapped_any as CPredicate,
            ),
            (
                "IN6_IS_ADDR_MC_RESERVED",
                libc::IN6_IS_ADDR_MC_RESERVED,
                in6_is_addr_mc_reserved,
            ),
            (
                "IN6_IS_ADDR_MC_SOLICITEDNODE",
                libc::IN6_IS_ADDR_MC_SOLICITEDNODE,
                in6_is_addr_mc_solicitednode,
            ),
            ("IN6_IS_ADDR_6TO4", libc::IN6_IS_ADDR_6TO4, in6_is_addr_6to4),
            (
                "IN6_IS_ADDR_LINKSCOPE",
                libc::IN6_IS_ADDR_LINKSCOPE,
                in6_is_addr_linkscope,
            ),
        ]);

        predicates
    }

    fn addr(s6_addr: [u8; 16]) -> in6_addr {
        let mut a: in6_addr = unsafe { std::mem::zeroed() };
        a.s6_addr = s6_addr;
        a
    }

    /// A deterministic xorshift generator, so failures are reproducible.
    struct Rng(u64);

    impl Rng {
        fn next(&mut self) -> u8 {
            self.0 ^= self.0 << 13;
            self.0 ^= self.0 >> 7;
            self.0 ^= self.0 << 17;
            self.0 as u8
        }

        fn bytes(&mut self) -> [u8; 16] {
            let mut b = [0; 16];
            b.iter_mut().for_each(|x| *x = self.next());
            b
        }
    }

    /// Addresses covering every prefix class: every combination of the first two bytes, plus
    /// addresses that are zero (or v4-mapped) up to some point and arbitrary after it.
    fn addresses() -> Vec<in6_addr> {
        let mut rng = Rng(0x2001_0db8_0000_0001);
        let mut addrs = Vec::new();

        for b0 in 0..=u8::MAX {
            for b1 in 0..=u8::MAX {
                let mut tail = rng.bytes();
                tail[0] = b0;
                tail[1] = b1;
                addrs.push(addr(tail));

                let mut zeros = [0; 16];
                zeros[0] = b0;
                zeros[1] = b1;
                addrs.push(addr(zeros));
                zeros[15] = 1;
                addrs.push(addr(zeros));
            }
        }

        for _ in 0..64 {
            let random = rng.bytes();
            for zeroed in 0..=16 {
                let mut b = random;
                b[..zeroed].fill(0);
                addrs.push(addr(b));

                if zeroed >= 10 {
                    b[10] = 0xff;
                    b[11] = 0xff;
                    addrs.push(addr(b));
                }
            }
        }

        // Single bytes in otherwise-zero addresses, for the last 32 bits
        for i in 8..16 {
            for v in [0x01, 0x02, 0x80, 0xff] {
                let mut b = [0; 16];
                b[i] = v;
                addrs.push(addr(b));
            }
        }

        // Solicited-node multicast addresses, ff02::1:ffXX:XXXX
        let solicited = [
            0xff, 0x02, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x01, 0xff, 0x12, 0x34, 0x56,
        ];
        addrs.push(addr(solicited));
        for i in 0..16 {
            let mut b = solicited;
            b[i] ^= 0x01;
            addrs.push(addr(b));
        }

        addrs
    }

    #[test]
    fn test_in6_is_addr() {
        let addrs = addresses();
        for (name, rust, c) in predicates() {
            for a in &addrs {
                assert_eq!(rust(a), unsafe { c(a) }, "{name}({:02x?})", a.s6_addr);
            }
        }
    }

    #[test]
    fn test_in6_are_addr_equal() {
        let addrs = addresses();
        for (i, a) in addrs.iter().enumerate().step_by(97) {
            let mut others = vec![*a, addrs[(i + 1) % addrs.len()]];
            for bit in 0..128 {
                let mut b = *a;
                b.s6_addr[bit / 8] ^= 1 << (bit % 8);
                others.push(b);
            }

            for b in &others {
                assert_eq!(
                    libc::IN6_ARE_ADDR_EQUAL(a, b),
                    unsafe { in6_are_addr_equal(a, b) },
                    "IN6_ARE_ADDR_EQUAL({:02x?}, {:02x?})",
                    a.s6_addr,
                    b.s6_addr
                );

                #[cfg(target_os = "illumos")]
                assert_eq!(
                    libc::IN6_ARE_6TO4_PREFIX_EQUAL(a, b),
                    unsafe { in6_are_6to4_prefix_equal(a, b) },
                    "IN6_ARE_6TO4_PREFIX_EQUAL({:02x?}, {:02x?})",
                    a.s6_addr,
                    b.s6_addr
                );
            }
        }
    }

    #[cfg(target_os = "android")]
    #[test]
    fn test_ipv6_addr_mc_scope() {
        for a in &addresses() {
            assert_eq!(
                libc::IPV6_ADDR_MC_SCOPE(a),
                unsafe { ipv6_addr_mc_scope(a) },
                "{:02x?}",
                a.s6_addr
            );
        }
    }

    #[cfg(target_vendor = "apple")]
    #[test]
    fn test_ipv6_addr_mc_flags() {
        for a in &addresses() {
            assert_eq!(
                libc::IPV6_ADDR_MC_FLAGS(a),
                unsafe { ipv6_addr_mc_flags(a) },
                "{:02x?}",
                a.s6_addr
            );
        }
    }
}
