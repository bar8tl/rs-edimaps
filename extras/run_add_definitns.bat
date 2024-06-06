echo off
echo 1.  delfor02
echo 2.  delvry03
echo 3.  delvry05
echo 4.  delvry07
echo 5.  invoic01
echo 6.  invoic02
echo 7.  orders01
echo 8.  orders05
echo 9.  seqjit03
echo 10. systat01
echo 11. y04c_shp_s09_asn
echo 12. y04vdelvry03
echo 13. y04vorders05
echo 14. y04vseqjit03
echo 15. z48v_invoic02_ex
echo 16. /rb04/yc1_pexr2002
echo 17. /rb04/yp3_delvry05_824
echo 18. /rb04/yp3_delvry_rbna
echo 29. /rb04/yp3_delvry_rbna_vw
echo 20. /rb04/yp3_invoic02
echo 21. /rb04/yp3_invoic02_cu
echo 22. /rb04/yp3_invoic02_intrace
echo 23. /rb04/yp3_invoic02_it
echo 24. /rb04/yp3_invoic02_xml
echo 90. all in list above
echo 99. exit
set /p opt=Enter option code:
IF %opt% == 1  goto s01
IF %opt% == 2  goto s02
IF %opt% == 3  goto s03
IF %opt% == 4  goto s04
IF %opt% == 5  goto s05
IF %opt% == 6  goto s06
IF %opt% == 7  goto s07
IF %opt% == 8  goto s08
IF %opt% == 9  goto s09
IF %opt% == 10 goto s10
IF %opt% == 11 goto s11
IF %opt% == 12 goto s12
IF %opt% == 13 goto s13
IF %opt% == 14 goto s14
IF %opt% == 15 goto s15
IF %opt% == 16 goto s16
IF %opt% == 17 goto s17
IF %opt% == 18 goto s18
IF %opt% == 19 goto s19
IF %opt% == 20 goto s20
IF %opt% == 21 goto s21
IF %opt% == 22 goto s22
IF %opt% == 23 goto s23
IF %opt% == 24 goto s24
IF %opt% == 90 goto s90
IF %opt% == 99 goto exit

:s01
echo on
..\bin\edimaps add -d delfor02.txt
goto exit

:s02
echo on
..\bin\edimaps add -d delvry03.txt
goto exit

:s03
echo on
..\bin\edimaps add -d delvry05.txt
goto exit

:s04
echo on
..\bin\edimaps add -d delvry07.txt
goto exit

:s05
echo on
..\bin\edimaps add -d invoic01.txt
goto exit

:s06
echo on
..\bin\edimaps add -d invoic02.txt
goto exit

:s07
echo on
..\bin\edimaps add -d orders01.txt
goto exit

:s08
echo on
..\bin\edimaps add -d orders05.txt
goto exit

:s09
echo on
..\bin\edimaps add -d seqjit03.txt
goto exit

:s10
echo on
..\bin\edimaps add -d systat01.txt
goto exit

:s11
echo on
..\bin\edimaps add -d y04c_shp_s09_asn.txt
goto exit

:s12
echo on
..\bin\edimaps add -d y04vdelvry03.txt
goto exit

:s13
echo on
..\bin\edimaps add -d y04vorders05.txt
goto exit

:s14
echo on
..\bin\edimaps add -d y04vseqjit03.txt
goto exit

:s15
echo on
..\bin\edimaps add -d z48v_invoic02_ex.txt
goto exit

:s16
echo on
..\bin\edimaps add -d _-rb04_-yc1_pexr2002.txt
goto exit

:s17
echo on
..\bin\edimaps add -d _-rb04_-yp3_delvry05_824.txt
goto exit

:s18
echo on
..\bin\edimaps add -d _-rb04_-yp3_delvry_rbna.txt
goto exit

:s19
echo on
..\bin\edimaps add -d _-rb04_-yp3_delvry_rbna_vw.txt
goto exit

:s20
echo on
..\bin\edimaps add -d _-rb04_-yp3_invoic02.txt
goto exit

:s21
echo on
..\bin\edimaps add -d _-rb04_-yp3_invoic02_cu.txt
goto exit

:s22
echo on
..\bin\edimaps add -d _-rb04_-yp3_invoic02_intrace.txt
goto exit

:s23
echo on
..\bin\edimaps add -d _-rb04_-yp3_invoic02_it.txt
goto exit

:s24
echo on
..\bin\edimaps add -d _-rb04_-yp3_invoic02_xml.txt
goto exit

:s90
echo on
..\bin\edimaps add -d delfor02.txt
..\bin\edimaps add -d delvry03.txt
..\bin\edimaps add -d delvry05.txt
..\bin\edimaps add -d delvry07.txt
..\bin\edimaps add -d invoic01.txt
..\bin\edimaps add -d invoic02.txt
..\bin\edimaps add -d orders01.txt
..\bin\edimaps add -d orders05.txt
..\bin\edimaps add -d seqjit03.txt
..\bin\edimaps add -d systat01.txt
..\bin\edimaps add -d y04c_shp_s09_asn.txt
..\bin\edimaps add -d y04vdelvry03.txt
..\bin\edimaps add -d y04vorders05.txt
..\bin\edimaps add -d y04vseqjit03.txt
..\bin\edimaps add -d z48v_invoic02_ex.txt
..\bin\edimaps add -d _-rb04_-yc1_pexr2002.txt
..\bin\edimaps add -d _-rb04_-yp3_delvry05_824.txt
..\bin\edimaps add -d _-rb04_-yp3_delvry_rbna.txt
..\bin\edimaps add -d _-rb04_-yp3_delvry_rbna_vw.txt
..\bin\edimaps add -d _-rb04_-yp3_invoic02.txt
..\bin\edimaps add -d _-rb04_-yp3_invoic02_cu.txt
..\bin\edimaps add -d _-rb04_-yp3_invoic02_intrace.txt
..\bin\edimaps add -d _-rb04_-yp3_invoic02_it.txt
..\bin\edimaps add -d _-rb04_-yp3_invoic02_xml.txt
goto exit

:exit
pause
