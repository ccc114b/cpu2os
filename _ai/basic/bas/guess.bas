10  LET T = 42
20  PRINT "猜猜看我的幸運數字？"
30  INPUT G
40  IF G = T THEN 80
50  IF G < T THEN PRINT "太小了！"
60  IF G > T THEN PRINT "太大了！"
70  GOTO 30
80  PRINT "恭喜！你猜對了。"
90  END