#include <stdbool.h>
#include <sys/types.h>
#include <netinet/in.h>

// Since the IN6_IS_ADDR_* family are macros instead of functions, they aren't
// available to FFI.  libc must reimplement them, which is error-prone.  This
// file provides FFI access to the actual macros so they can be tested against
// the Rust reimplementations.

#define IN6_PREDICATE(name, macro) \
    bool name(const struct in6_addr *a) { return macro(a); }

IN6_PREDICATE(in6_is_addr_unspecified, IN6_IS_ADDR_UNSPECIFIED)
IN6_PREDICATE(in6_is_addr_loopback, IN6_IS_ADDR_LOOPBACK)
IN6_PREDICATE(in6_is_addr_multicast, IN6_IS_ADDR_MULTICAST)
IN6_PREDICATE(in6_is_addr_linklocal, IN6_IS_ADDR_LINKLOCAL)
IN6_PREDICATE(in6_is_addr_sitelocal, IN6_IS_ADDR_SITELOCAL)
IN6_PREDICATE(in6_is_addr_v4mapped, IN6_IS_ADDR_V4MAPPED)
IN6_PREDICATE(in6_is_addr_v4compat, IN6_IS_ADDR_V4COMPAT)
IN6_PREDICATE(in6_is_addr_mc_nodelocal, IN6_IS_ADDR_MC_NODELOCAL)
IN6_PREDICATE(in6_is_addr_mc_linklocal, IN6_IS_ADDR_MC_LINKLOCAL)
IN6_PREDICATE(in6_is_addr_mc_sitelocal, IN6_IS_ADDR_MC_SITELOCAL)
IN6_PREDICATE(in6_is_addr_mc_orglocal, IN6_IS_ADDR_MC_ORGLOCAL)
IN6_PREDICATE(in6_is_addr_mc_global, IN6_IS_ADDR_MC_GLOBAL)

bool in6_are_addr_equal(const struct in6_addr *a, const struct in6_addr *b) {
    return IN6_ARE_ADDR_EQUAL(a, b);
}

// Platform-specific extensions

#ifdef IN6_IS_ADDR_ULA
IN6_PREDICATE(in6_is_addr_ula, IN6_IS_ADDR_ULA)
#endif

#ifdef IN6_IS_ADDR_UNIQUE_LOCAL
IN6_PREDICATE(in6_is_addr_unique_local, IN6_IS_ADDR_UNIQUE_LOCAL)
#endif

#ifdef IN6_IS_ADDR_UNICAST_BASED_MULTICAST
IN6_PREDICATE(in6_is_addr_unicast_based_multicast, IN6_IS_ADDR_UNICAST_BASED_MULTICAST)
#endif

#ifdef IN6_IS_ADDR_MC_INTFACELOCAL
IN6_PREDICATE(in6_is_addr_mc_intfacelocal, IN6_IS_ADDR_MC_INTFACELOCAL)
#endif

#ifdef IN6_IS_ADDR_V4MAPPED_ANY
IN6_PREDICATE(in6_is_addr_v4mapped_any, IN6_IS_ADDR_V4MAPPED_ANY)
#endif

#ifdef IN6_IS_ADDR_MC_RESERVED
IN6_PREDICATE(in6_is_addr_mc_reserved, IN6_IS_ADDR_MC_RESERVED)
#endif

#ifdef IN6_IS_ADDR_MC_SOLICITEDNODE
IN6_PREDICATE(in6_is_addr_mc_solicitednode, IN6_IS_ADDR_MC_SOLICITEDNODE)
#endif

// Apple's version uses `s6_addr16`, which is only defined for the kernel
#if defined(IN6_IS_ADDR_6TO4) && !defined(__APPLE__)
IN6_PREDICATE(in6_is_addr_6to4, IN6_IS_ADDR_6TO4)
#endif

#ifdef IN6_IS_ADDR_LINKSCOPE
IN6_PREDICATE(in6_is_addr_linkscope, IN6_IS_ADDR_LINKSCOPE)
#endif

#ifdef IN6_ARE_6TO4_PREFIX_EQUAL
bool in6_are_6to4_prefix_equal(const struct in6_addr *a, const struct in6_addr *b) {
    return IN6_ARE_6TO4_PREFIX_EQUAL(a, b);
}
#endif

#ifdef IPV6_ADDR_MC_SCOPE
unsigned char ipv6_addr_mc_scope(const struct in6_addr *a) {
    return IPV6_ADDR_MC_SCOPE(a);
}
#endif

#ifdef IPV6_ADDR_MC_FLAGS
unsigned char ipv6_addr_mc_flags(const struct in6_addr *a) {
    return IPV6_ADDR_MC_FLAGS(a);
}
#endif
