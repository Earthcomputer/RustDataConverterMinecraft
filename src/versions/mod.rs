macro_rules! versions {
    ($(mod $version:ident);* $(;)?) => {
        $(
            pub(crate) mod $version;
        )*

        pub(crate) fn register_versions<T: rust_dataconverter_engine::Types + ?Sized>(types: &$crate::types::MinecraftTypesMut<T>) {
            $(
                $version::register(types);
            )*
        }
    }
}

versions! {
    mod v99;
    mod v100;
    mod v101;
    mod v102;
    mod v105;
    mod v106;
    mod v107;
    mod v108;
    mod v109;
    mod v110;
    mod v111;
    mod v113;
    mod v135;
    mod v143;
    mod v147;
    mod v165;
    mod v501;
    mod v502;
    mod v505;
    mod v700;
    mod v701;
    mod v702;
    mod v703;
    mod v704;
    mod v705;
    mod v804;
    mod v806;
    mod v808;
    mod v813;
    mod v816;
    mod v820;
    mod v1022;
    mod v1125;
    mod v1344;
    mod v1446;
    mod v1450;
    mod v1451;
    mod v1456;
    mod v1458;
    mod v1460;
    mod v1466;
    mod v1470;
    mod v1474;
    mod v1475;
    mod v1480;
    mod v1483;
    mod v1484;
    mod v1486;
    mod v1487;
    mod v1488;
    mod v1490;
    mod v1492;
    mod v1494;
    mod v1496;
    mod v1500;
    mod v1501;
    mod v1502;
    mod v1506;
    mod v1510;
    mod v1514;
    mod v1515;
    mod v1624;
    mod v1800;
    mod v1801;
    mod v1802;
    mod v1803;
    mod v1904;
    mod v1905;
    mod v1906;
    mod v1911;
    mod v1914;
    mod v1917;
    mod v1918;
    mod v1920;
    mod v1925;
    mod v1928;
    mod v1929;
    mod v1931;
    mod v1936;
    mod v1946;
    mod v1948;
    mod v1953;
    mod v1955;
    mod v1961;
    mod v1963;
    mod v2100;
    mod v2202;
    mod v2209;
    mod v2211;
    mod v2218;
    mod v2501;
    mod v2502;
    mod v2503;
    mod v2505;
    mod v2508;
    mod v2509;
    mod v2511;
    mod v2514;
    mod v2516;
    mod v2518;
    mod v2519;
    mod v2522;
    mod v2523;
    mod v2527;
    mod v2528;
    mod v2529;
    mod v2531;
    mod v2533;
    mod v2535;
    mod v2550;
    mod v2551;
    mod v2552;
    mod v2553;
    mod v2558;
}
