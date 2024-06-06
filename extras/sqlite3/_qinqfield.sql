SELECT a.mapid, a.chgnr, b.ctmrl, b.messg, b.mvers, b.idocm, a.sgmid, a.targt, a.sourc, a.rcond, a.commt, a.sampl
FROM fields AS a LEFT JOIN indix AS b ON a.mapid = b.mapid and a.chgnr = b.chgnr
where a.targt = 'BSTDK' and b.messg = '830' and b.mvers = '4010' order by a.sgmid, a.grpid, a.targt;