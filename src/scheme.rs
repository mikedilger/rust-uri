
use std::fmt;
use std::fmt::Display;
use std::str::FromStr;
use std::ascii::AsciiExt;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum SchemeStatus {
    Permanent,
    Provisional,
    Historical,
    Unregistered,
}

macro_rules! def_schemes {
    ($($id:ident, $str:expr, $desc:expr, $refr:expr, $status:ident);+) => (
        #[derive(PartialEq, Eq, Clone, Debug)]
        pub enum Scheme {
            $($id),+,
            Unknown(String)
        }

        impl Display for Scheme {
            fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
                match *self {
                    $(Scheme::$id => write!(f, "{}", $str)),+,
                    Scheme::Unknown(ref s) => write!(f, "{}", s.to_ascii_lowercase())
                }
            }
        }

        impl FromStr for Scheme {
            type Err = ();
            fn from_str(s: &str) -> Result<Scheme, ()> {
                let scheme = match s.to_ascii_lowercase().as_slice() {
                    $($str => Scheme::$id),+,
                    _ => return Err(())
                };
                Ok(scheme)
            }
        }

        impl Scheme {
            pub fn iana_status(&self) -> SchemeStatus {
                match *self {
                    $(Scheme::$id => SchemeStatus::$status),+,
                    Scheme::Unknown(_) => return SchemeStatus::Unregistered
                }
            }

            pub fn description<'a>(&'a self) -> &'a str {
                match *self {
                    $(Scheme::$id => $desc),+,
                    Scheme::Unknown(_) => return ""
                }
            }

            pub fn iana_reference<'a>(&'a self) -> &'a str {
                match *self {
                    $(Scheme::$id => $refr),+,
                    Scheme::Unknown(_) => return ""
                }
            }
        }
    )
}

// This data is from http://www.iana.org/assignments/uri-schemes/uri-schemes.xhtml
// last updated as of 2014-02-07
def_schemes!(
    Aaa, "aaa", "Diameter Protocol", "[RFC6733]", Permanent;
    Aaas, "aaas", "Diameter Protocol with Secure Transport", "[RFC6733]", Permanent;
    About, "about", "about", "[RFC6694]", Permanent;
    Acap, "acap", "application configuration access protocol", "[RFC2244]", Permanent;
    Acct, "acct", "acct", "[RFC-ietf-appsawg-acct-uri-06]", Permanent;
    Acr, "acr", "acr", "[OMA-OMNA]", Provisional;
    Adiumxtra, "adiumxtra", "adiumxtra", "[Dave_Thaler]", Provisional;
    Afp, "afp", "afp", "[Dave_Thaler]", Provisional;
    Afs, "afs", "Andrew File System global file names", "[RFC1738]", Provisional;
    Aim, "aim", "aim", "[Dave_Thaler]", Provisional;
    Apt, "apt", "apt", "[Dave_Thaler]", Provisional;
    Attachment, "attachment", "attachment", "[Dave_Thaler]", Provisional;
    Aw, "aw", "aw", "[Dave_Thaler]", Provisional;
    Barion, "barion", "barion", "[Bíró_Tamás]", Provisional;
    Beshare, "beshare", "beshare", "[Dave_Thaler]", Provisional;
    Bitcoin, "bitcoin", "bitcoin", "[Dave_Thaler]", Provisional;
    Bolo, "bolo", "bolo", "[Dave_Thaler]", Provisional;
    Callto, "callto", "callto", "[Alexey_Melnikov]", Provisional;
    Cap, "cap", "Calendar Access Protocol", "[RFC4324]", Permanent;
    Chrome, "chrome", "chrome", "[Dave_Thaler]", Provisional;
    ChromeExtension, "chrome-extension", "chrome-extension", "[Dave_Thaler]", Provisional;
    Cid, "cid", "content identifier", "[RFC2392]", Permanent;
    Coap, "coap", "coap", "[RFC7252]", Permanent;
    Coaps, "coaps", "coaps", "[RFC7252]", Permanent;
    ComEventbriteAttendee, "com-eventbrite-attendee", "com-eventbrite-attendee", "[Bob_Van_Zant]", Provisional;
    Content, "content", "content", "[Dave_Thaler]", Provisional;
    Crid, "crid", "TV-Anytime Content Reference Identifier", "[RFC4078]", Permanent;
    Cvs, "cvs", "cvs", "[Dave_Thaler]", Provisional;
    Data, "data", "data", "[RFC2397]", Permanent;
    Dav, "dav", "dav", "[RFC4918]", Permanent;
    Dict, "dict", "dictionary service protocol", "[RFC2229]", Permanent;
    DlnaPlaycontanier, "dlna-playcontainer", "dlna-playcontainer", "[DLNA]", Provisional;
    DlnaPlaysingle, "dlna-playsingle", "dlna-playsingle", "[DLNA]", Provisional;
    Dns, "dns", "Domain Name System", "[RFC4501]", Permanent;
    Dtn, "dtn", "DTNRG research and development", "[RFC5050]", Provisional;
    Dvb, "dvb", "dvb", "[draft-mcroberts-uri-dvb]", Provisional;
    Ed2k, "ed2k", "ed2k", "[Dave_Thaler]", Provisional;
    Facetime, "facetime", "facetime", "[Dave_Thaler]", Provisional;
    Fax, "fax", "fax", "[RFC2806][RFC3966]", Historical;
    Feed, "feed", "feed", "[Dave_Thaler]", Provisional;
    Feedready, "feedready", "feedready", "[Mirko_Nosenzo]", Provisional;
    File, "file", "Host-specific file names", "[RFC1738]", Permanent;
    Finger, "finger", "finger", "[Dave_Thaler]", Provisional;
    Fish, "fish", "fish", "[Dave_Thaler]", Provisional;
    Ftp, "ftp", "File Transfer Protocol", "[RFC1738]", Permanent;
    Geo, "geo", "Geographic Locations", "[RFC5870]", Permanent;
    Gg, "gg", "gg", "[Dave_Thaler]", Provisional;
    Git, "git", "git", "[Dave_Thaler]", Provisional;
    Gizmoproject, "gizmoproject", "gizmoproject", "[Dave_Thaler]", Provisional;
    Go, "go", "go", "[RFC3368]", Permanent;
    Gopher, "gopher", "The Gopher Protocol", "[RFC4266]", Permanent;
    Gtalk, "gtalk", "gtalk", "[Dave_Thaler]", Provisional;
    H323, "h323", "H.323", "[RFC3508]", Permanent;
    Ham, "ham", "ham", "[RFC7046]", Provisional;
    Hcp, "hcp", "hcp", "[Alexey_Melnikov]", Provisional;
    Http, "http", "Hypertext Transfer Protocol", "[RFC7230, Section 2.7.1]", Permanent;
    Https, "https", "Hypertext Transfer Protocol Secure", "[RFC7230, Section 2.7.2]", Permanent;
    Iax, "iax", "Inter-Asterisk eXchange Version 2", "[RFC5456]", Permanent;
    Icap, "icap", "Internet Content Adaptation Protocol", "[RFC3507]", Permanent;
    Icon, "icon", "icon", "[draft-lafayette-icon-uri-scheme]", Provisional;
    Imap, "imap", "internet message access protocol", "[RFC5092]", Permanent;
    Im, "im", "Instant Messaging", "[RFC3860]", Permanent;
    Info, "info", "Information Assets with Identifiers in Public Namespaces", "[RFC4452]", Permanent;
    Ipn, "ipn", "ipn", "[RFC6260]", Provisional;
    Ipp, "ipp", "Internet Printing Protocol", "[RFC3510]", Permanent;
    Ipps, "ipps", "Internet Printing Protocol over HTTPS", "[RFC-mcdonald-ipps-uri-scheme-18]", Permanent;
    Irc6, "irc6", "irc6", "[Dave_Thaler]", Provisional;
    Irc, "irc", "irc", "[Dave_Thaler]", Provisional;
    Ircs, "ircs", "ircs", "[Dave_Thaler]", Provisional;
    IrisBeep, "iris.beep", "iris.beep", "[RFC3983]", Permanent;
    Iris, "iris", "Internet Registry Information Service", "[RFC3981]", Permanent;
    IrisLwz, "iris.lwz", "iris.lwz", "[RFC4993]", Permanent;
    IrisXpc, "iris.xpc", "iris.xpc", "[RFC4992]", Permanent;
    IrisXpcs, "iris.xpcs", "iris.xpcs", "[RFC4992]", Permanent;
    Itms, "itms", "itms", "[Dave_Thaler]", Provisional;
    Jabber, "jabber", "jabber", "[Peter_Saint-Andre]", Permanent;
    Jar, "jar", "jar", "[Dave_Thaler]", Provisional;
    Jms, "jms", "Java Message Service", "[RFC6167]", Provisional;
    Keyparc, "keyparc", "keyparc", "[Dave_Thaler]", Provisional;
    Lastfm, "lastfm", "lastfm", "[Dave_Thaler]", Provisional;
    Ldap, "ldap", "Lightweight Directory Access Protocol", "[RFC4516]", Permanent;
    Ldaps, "ldaps", "ldaps", "[Dave_Thaler]", Provisional;
    Magnet, "magnet", "magnet", "[Dave_Thaler]", Provisional;
    Mailserver, "mailserver", "Access to data available from mail servers", "[RFC6196]", Historical;
    Mailto, "mailto", "Electronic mail address", "[RFC6068]", Permanent;
    Maps, "maps", "maps", "[Dave_Thaler]", Provisional;
    Market, "market", "market", "[Dave_Thaler]", Provisional;
    Message, "message", "message", "[Dave_Thaler]", Provisional;
    Mid, "mid", "message identifier", "[RFC2392]", Permanent;
    Mms, "mms", "mms", "[Alexey_Melnikov]", Provisional;
    Modem, "modem", "modem", "[RFC2806][RFC3966]", Historical;
    MsHelp, "ms-help", "ms-help", "[Alexey_Melnikov]", Provisional;
    Msnim, "msnim", "msnim", "[Alexey_Melnikov]", Provisional;
    Msrp, "msrp", "Message Session Relay Protocol", "[RFC4975]", Permanent;
    Msrps, "msrps", "Message Session Relay Protocol Secure", "[RFC4975]", Permanent;
    MsSettingsPower, "ms-settings-power", "ms-settings-power", "[urischemeowners_at_microsoft.com]", Provisional;
    Mtqp, "mtqp", "Message Tracking Query Protocol", "[RFC3887]", Permanent;
    Mumble, "mumble", "mumble", "[Dave_Thaler]", Provisional;
    Mupdate, "mupdate", "Mailbox Update (MUPDATE) Protocol", "[RFC3656]", Permanent;
    Mvn, "mvn", "mvn", "[Dave_Thaler]", Provisional;
    Mysql, "mysql", "MySql Database Connection", "", Unregistered;
    News, "news", "USENET news", "[RFC5538]", Permanent;
    Nfs, "nfs", "network file system protocol", "[RFC2224]", Permanent;
    Nih, "nih", "nih", "[RFC6920]", Permanent;
    Ni, "ni", "ni", "[RFC6920]", Permanent;
    Nntp, "nntp", "USENET news using NNTP access", "[RFC5538]", Permanent;
    Notes, "notes", "notes", "[Dave_Thaler]", Provisional;
    Oid, "oid", "oid", "[draft-larmouth-oid-iri]", Provisional;
    Opaquelocktoken, "opaquelocktoken", "opaquelocktokent", "[RFC4918]", Permanent;
    Pack, "pack", "pack", "[draft-shur-pack-uri-scheme]", Historical;
    Palm, "palm", "palm", "[Dave_Thaler]", Provisional;
    Paparazzi, "paparazzi", "paparazzi", "[Dave_Thaler]", Provisional;
    Pkcs11, "pkcs11", "pkcs11", "[draft-pechanec-pkcs11uri]", Provisional;
    Platform, "platform", "platform", "[Dave_Thaler]", Provisional;
    Pop, "pop", "Post Office Protocol v3", "[RFC2384]", Permanent;
    Postgres, "postgres", "Postgres Database Connection", "", Unregistered;
    Postgresql, "postgresql", "Postgres Database Connection", "", Unregistered;
    Pres, "pres", "Presence", "[RFC3859]", Permanent;
    Prospero, "prospero", "Prospero Directory Service", "[RFC4157]", Historical;
    Proxy, "proxy", "proxy", "[Dave_Thaler]", Provisional;
    Psyc, "psyc", "psyc", "[Dave_Thaler]", Provisional;
    Query, "query", "query", "[Dave_Thaler]", Provisional;
    Reload, "reload", "reload", "[RFC6940]", Permanent;
    Resource, "resource", "resource", "[Dave_Thaler]", Provisional;
    Res, "res", "res", "[Alexey_Melnikov]", Provisional;
    Rmi, "rmi", "rmi", "[Dave_Thaler]", Provisional;
    Rsync, "rsync", "rsync", "[RFC5781]", Provisional;
    Rtmfp, "rtmfp", "rtmfp", "[RFC7425]", Provisional;
    Rtmp, "rtmp", "rtmp", "[Dave_Thaler]", Provisional;
    Rtsp, "rtsp", "Real-time Streaming Protocol (RTSP)", "[RFC2326][RFC-ietf-mmusic-rfc2326bis-40]", Permanent;
    Rtsps, "rtsps", "Real-time Streaming Protocol (RTSP) over TLS", "[RFC2326][RFC-ietf-mmusic-rfc2326bis-40]", Permanent;
    Rtspu, "rtspu", "Real-time Streaming Protocol (RTSP) over unreliable datagram transport", "[RFC2326]", Permanent;
    Secondlife, "secondlife", "query", "[Dave_Thaler]", Provisional;
    Service, "service", "service location", "[RFC2609]", Permanent;
    Session, "session", "session", "[RFC6787]", Permanent;
    Sftp, "sftp", "query", "[Dave_Thaler]", Provisional;
    Sgn, "sgn", "sgn", "[Dave_Thaler]", Provisional;
    Shttp, "shttp", "Secure Hypertext Transfer Protocol", "[RFC2660]", Permanent;
    Sieve, "sieve", "ManageSieve Protocol", "[RFC5804]", Permanent;
    Sip, "sip", "session initiation protocol", "[RFC3261]", Permanent;
    Sips, "sips", "secure session initiation protocol", "[RFC3261]", Permanent;
    Skype, "skype", "skype", "[Alexey_Melnikov]", Provisional;
    Smb, "smb", "smb", "[Dave_Thaler]", Provisional;
    Sms, "sms", "Short Message Service", "[RFC5724]", Permanent;
    Smtp, "smtp", "smtp", "[draft-melnikov-smime-msa-to-mda]", Provisional;
    Snews, "snews", "NNTP over SSL/TLS", "[RFC5538]", Historical;
    Snmp, "snmp", "Simple Network Management Protocol", "[RFC4088]", Permanent;
    SoapBeep, "soap.beep", "soap.beep", "[RFC4227]", Permanent;
    SoapBeeps, "soap.beeps", "soap.beeps", "[RFC4227]", Permanent;
    Soldat, "soldat", "soldat", "[Dave_Thaler]", Provisional;
    Spotify, "spotify", "spotify", "[Dave_Thaler]", Provisional;
    Ssh, "ssh", "ssh", "[Dave_Thaler]", Provisional;
    Steam, "steam", "steam", "[Dave_Thaler]", Provisional;
    Stuns, "stuns", "stuns", "[RFC7064]", Permanent;
    Stun, "stun", "stun", "[RFC7064]", Permanent;
    Submit, "submit", "submit", "[draft-melnikov-smime-msa-to-mda]", Provisional;
    Svn, "svn", "svn", "[Dave_Thaler]", Provisional;
    Tag, "tag", "tag", "[RFC4151]", Permanent;
    Teamspeak, "teamspeak", "teamspeak", "[Dave_Thaler]", Provisional;
    Teliaeid, "teliaeid", "teliaeid", "[Peter_Lewandowski]", Provisional;
    Telnet, "telnet", "Reference to interactive sessions", "[RFC4248]", Permanent;
    Tel, "tel", "telephone", "[RFC3966]", Permanent;
    Tftp, "tftp", "Trivial File Transfer Protocol", "[RFC3617]", Permanent;
    Things, "things", "things", "[Dave_Thaler]", Provisional;
    ThisMessage, "thismessage", "multipart/related relative reference resolution", "[RFC2557]", Permanent;
    Tip, "tip", "Transaction Internet Protocol", "[RFC2371]", Permanent;
    Tn3270, "tn3270", "Interactive 3270 emulation sessions", "[RFC6270]", Permanent;
    Turns, "turns", "turns", "[RFC7065]", Permanent;
    Turn, "turn", "turn", "[RFC7065]", Permanent;
    Tv, "tv", "TV Broadcasts", "[RFC2838]", Permanent;
    Udp, "udp", "udp", "[Dave_Thaler]", Provisional;
    Unreal, "unreal", "unreal", "[Dave_Thaler]", Provisional;
    Urn, "urn", "Uniform Resource Names", "[RFC2141][IANA registry urn-namespaces]", Permanent;
    Ut2004, "ut2004", "ut2004", "[Dave_Thaler]", Provisional;
    Vemmi, "vemmi", "versatile multimedia interface", "[RFC2122]", Permanent;
    Ventrilo, "ventrilo", "ventrilo", "[Dave_Thaler]", Provisional;
    Videotex, "videotex", "videotex", "[draft-mavrakis-videotex-url-spec][RFC2122][RFC3986]", Historical;
    ViewSource, "view-source", "view-source", "[Mykyta_Yevstifeyev]", Provisional;
    Wais, "wais", "Wide Area Information Servers", "[RFC4156]", Historical;
    Webcal, "webcal", "webcal", "[Dave_Thaler]", Provisional;
    Wss, "wss", "Encrypted WebSocket connections", "[RFC6455]", Permanent;
    Ws, "ws", "WebSocket connections", "[RFC6455]", Permanent;
    Wtai, "wtai", "wtai", "[Dave_Thaler]", Provisional;
    Wyciwyg, "wyciwyg", "wyciwyg", "[Dave_Thaler]", Provisional;
    XconUserid, "xcon-userid", "xcon-userid", "[RFC6501]", Permanent;
    Xcon, "xcon", "xcon", "[RFC6501]", Permanent;
    Xfire, "xfire", "xfire", "[Dave_Thaler]", Provisional;
    XmlrpcBeeps, "xmlrpc.beeps", "xmlrpc.beeps", "[RFC3529]", Permanent;
    XmlrpcBeep, "xmlrpc.beep", "xmlrpc.beep", "[RFC3529]", Permanent;
    Xmpp, "xmpp", "Extensible Messaging and Presence Protocol", "[RFC5122]", Permanent;
    Xri, "xri", "xri", "[Dave_Thaler]", Provisional;
    Ymsgr, "ymsgr", "ymsgr", "[Dave_Thaler]", Provisional;
    Z3950r, "z39.50r", "Z39.50 Retrieval", "[RFC2056]", Permanent;
    Z3950s, "z39.50s", "Z39.50 Session", "[RFC2056]", Permanent;
    Z3950, "z39.50", "Z39.50 information access", "[RFC1738][RFC2056]", Historical
    );
