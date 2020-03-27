# https://github.com/ghewgill/pydkim/blob/master/dkim.py



def sign(
    message, selector, domain, privkey, identity=None,
    canonicalize=(Simple, Simple),
    include_headers=None, length=False, debuglog=None
):

    (headers, body) = rfc822_parse(message)

    m = re.search("--\n(.*?)\n--", privkey, re.DOTALL)
    if m is None:
        raise KeyFormatError("Private key not found")
    try:
        pkdata = base64.b64decode(m.group(1))
    except TypeError, e:
        raise KeyFormatError(str(e))
    if debuglog is not None:
        print >>debuglog, " ".join("%02x" % ord(x) for x in pkdata)
    pka = asn1_parse(ASN1_RSAPrivateKey, pkdata)
    pk = {
        'version': pka[0][0],
        'modulus': pka[0][1],
        'publicExponent': pka[0][2],
        'privateExponent': pka[0][3],
        'prime1': pka[0][4],
        'prime2': pka[0][5],
        'exponent1': pka[0][6],
        'exponent2': pka[0][7],
        'coefficient': pka[0][8],
    }

    if identity is not None and not identity.endswith(domain):
        raise ParameterError("identity must end with domain")

    headers = canonicalize[0].canonicalize_headers(headers)

    if include_headers is None:
        include_headers = [x[0].lower() for x in headers]
    else:
        include_headers = [x.lower() for x in include_headers]
    sign_headers = [x for x in headers if x[0].lower() in include_headers]

    body = canonicalize[1].canonicalize_body(body)

    if HaveSHA256:
        h = hashlib.sha256()
    else:
        h = sha.sha()
    h.update(body)
    bodyhash = base64.b64encode(h.digest())

    sigfields = [x for x in [
        ('v', "1"),
        ('a', HaveSHA256 and "rsa-sha256" or "rsa-sha1"),
        ('c', "%s/%s" % (canonicalize[0].name, canonicalize[1].name)),
        ('d', domain),
        ('i', identity or "@"+domain),
        length and ('l', len(body)),
        ('q', "dns/txt"),
        ('s', selector),
        ('t', str(int(time.time()))),
        ('h', " : ".join(x[0] for x in sign_headers)),
        ('bh', bodyhash),
        ('b', ""),
    ] if x]
    sig = "DKIM-Signature: " + "; ".join("%s=%s" % x for x in sigfields)

    sig = fold(sig)

    if debuglog is not None:
        print >>debuglog, "sign headers:", sign_headers + [("DKIM-Signature", " "+"; ".join("%s=%s" % x for x in sigfields))]
    if HaveSHA256:
        h = hashlib.sha256()
    else:
        h = sha.sha()
    for x in sign_headers:
        h.update(x[0])
        h.update(":")
        h.update(x[1])
    h.update(sig)
    d = h.digest()
    if debuglog is not None:
        print >>debuglog, "sign digest:", " ".join("%02x" % ord(x) for x in d)

    dinfo = asn1_build(
        (SEQUENCE, [
            (SEQUENCE, [
                (OBJECT_IDENTIFIER, HaveSHA256 and HASHID_SHA256 or HASHID_SHA1),
                (NULL, None),
            ]),
            (OCTET_STRING, d),
        ])
    )
    modlen = len(int2str(pk['modulus']))
    if len(dinfo)+3 > modlen:
        raise ParameterError("Hash too large for modulus")
    sig2 = int2str(pow(str2int("\x00\x01"+"\xff"*(modlen-len(dinfo)-3)+"\x00"+dinfo), pk['privateExponent'], pk['modulus']), modlen)
    sig += base64.b64encode(''.join(sig2))

    return sig + "\r\n"

