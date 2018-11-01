
--Create the Day6 schema
IF NOT EXISTS(SELECT * FROM sys.schemas WHERE [name] = 'Day6')
BEGIN
	EXEC sp_executesql N'CREATE SCHEMA Day6';
END

--Create the Day6.LightGrid table
DROP TABLE IF EXISTS Day6.LightGrid;
CREATE TABLE Day6.LightGrid(
	rowPosition		INT NOT NULL,
	columnPosition	INT NOT NULL,
	lightState		TINYINT DEFAULT(0),
	CONSTRAINT PK_LightGrid PRIMARY KEY (rowPosition, columnPosition)
);
GO

--Populate Day6.LightGrid with data representing the starting point of a 1,000 x 1,000 grid with all lights off
WITH t1(n) AS
(
	SELECT 1 UNION ALL SELECT 1 UNION ALL SELECT 1 UNION ALL
	SELECT 1 UNION ALL SELECT 1 UNION ALL SELECT 1 UNION ALL
	SELECT 1 UNION ALL SELECT 1 UNION ALL SELECT 1 UNION ALL SELECT 1
),
t2(n) AS (SELECT 1 FROM t1 CROSS JOIN t1 AS a),
numbers(n) AS (
	SELECT ROW_NUMBER() OVER(ORDER BY t2.n)
	FROM t2 
	CROSS JOIN (SELECT TOP 10 1 n FROM t2) AS a)
INSERT INTO Day6.LightGrid(rowPosition, columnPosition)
SELECT 
	n1.n AS rowPosition
,	n2.n AS columnPosition
FROM numbers n1
CROSS JOIN numbers n2;
GO

--Create proc which given an instruction line will make the require adjustments to the lights for part1
CREATE OR ALTER PROC Day6.LightsInstructionPart1(@instruction VARCHAR(100)) 
AS
BEGIN
	DECLARE @action TINYINT
	DECLARE @tempinstruction VARCHAR(100)

	--Establishes an action type to be used
	SET @action = 
		CASE 
			WHEN @instruction LIKE 'turn on%' THEN 1
			WHEN @instruction LIKE 'turn off%' THEN 0
			ELSE 2
		END

	DECLARE @startRow INT, @startCol INT, @endRow INT,  @endCol INT;

	SET @tempinstruction = RIGHT(@instruction, LEN(@instruction) - CASE @action WHEN 1 THEN 8 WHEN 0 THEN 9 ELSE 7 END);

	--Get the startRow and then strips down the rest of the instruction
	SET @startRow = TRY_PARSE(LEFT(@tempinstruction, CHARINDEX(',', @tempinstruction)) AS INT);
	SET @tempinstruction = RIGHT(@tempinstruction, LEN(@tempinstruction) - CHARINDEX(',', @tempinstruction));

	--Get the startCol and then strips down the rest of the instruction
	SET @startCol = TRY_PARSE(LEFT(@tempinstruction, CHARINDEX('through', @tempinstruction) - 1) AS INT);
	SET @tempinstruction = RIGHT(@tempinstruction, LEN(@tempinstruction) - (CHARINDEX('through', @tempinstruction) + 7));

	--Get the endRow and endCol from the remaining instruction
	SET @endRow = TRY_PARSE(LEFT(@tempinstruction, CHARINDEX(',', @tempinstruction)) AS INT);
	SET @endCol = TRY_PARSE(RIGHT(@tempinstruction, LEN(@tempinstruction) - CHARINDEX(',', @tempinstruction)) AS INT);

	--Now update the grid to reflect the changes in the instruction
	UPDATE Day6.LightGrid
	SET lightState =
		CASE @action
			WHEN 2 THEN 1 ^ CAST(lightState AS BIT)
			ELSE @action
		END
	WHERE rowPosition BETWEEN @startRow AND @endRow
	AND columnPosition BETWEEN @startCol AND @endCol
END;
GO


--Create proc which given an instruction line will make the require adjustments to the lights for part2
CREATE OR ALTER PROC Day6.LightsInstructionPart2(@instruction VARCHAR(100)) 
AS
BEGIN
	DECLARE @action TINYINT
	DECLARE @tempinstruction VARCHAR(100)

	--Establishes an action type to be used
	SET @action = 
		CASE 
			WHEN @instruction LIKE 'turn on%' THEN 1
			WHEN @instruction LIKE 'turn off%' THEN 0
			ELSE 2
		END

	DECLARE @startRow INT, @startCol INT, @endRow INT,  @endCol INT;

	SET @tempinstruction = RIGHT(@instruction, LEN(@instruction) - CASE @action WHEN 1 THEN 8 WHEN 0 THEN 9 ELSE 7 END);

	--Get the startRow and then strips down the rest of the instruction
	SET @startRow = TRY_PARSE(LEFT(@tempinstruction, CHARINDEX(',', @tempinstruction)) AS INT);
	SET @tempinstruction = RIGHT(@tempinstruction, LEN(@tempinstruction) - CHARINDEX(',', @tempinstruction));

	--Get the startCol and then strips down the rest of the instruction
	SET @startCol = TRY_PARSE(LEFT(@tempinstruction, CHARINDEX('through', @tempinstruction) - 1) AS INT);
	SET @tempinstruction = RIGHT(@tempinstruction, LEN(@tempinstruction) - (CHARINDEX('through', @tempinstruction) + 7));

	--Get the endRow and endCol from the remaining instruction
	SET @endRow = TRY_PARSE(LEFT(@tempinstruction, CHARINDEX(',', @tempinstruction)) AS INT);
	SET @endCol = TRY_PARSE(RIGHT(@tempinstruction, LEN(@tempinstruction) - CHARINDEX(',', @tempinstruction)) AS INT);

	--Now update the grid to reflect the changes in the instruction
	UPDATE Day6.LightGrid
	SET lightState =
		CASE @action
			WHEN 1 THEN lightState + 1
			WHEN 0 THEN CASE WHEN lightState > 0 THEN lightState - 1 ELSE 0 END
			WHEN 2 THEN lightState + 2
		END
	WHERE rowPosition BETWEEN @startRow AND @endRow
	AND columnPosition BETWEEN @startCol AND @endCol
END;
GO

--Create an instructions table
DROP TABLE IF EXISTS Day6.Instructions;
CREATE TABLE Day6.Instructions(
	position		INT IDENTITY(1,1) PRIMARY KEY,
	instruction VARCHAR(100)
);
GO

--Bulk insert data from the input file into #Instructions
DROP TABLE IF EXISTS #Instructions;
CREATE TABLE #Instructions(Instruction VARCHAR(100));
BULK INSERT #Instructions FROM 'D:\AdventOfCode\adventofcode2015\day6\day6input.txt'
WITH (FIELDTERMINATOR ='|',ROWTERMINATOR ='\n');

--Copy the data from #Instructions to Day6.Instructions
INSERT INTO Day6.Instructions(Instruction)
SELECT Instruction
FROM #Instructions;

/*
RUN THROUGH THE INSTRUCTIONS FOR PART 1
*/
--Before running through the instructions make sure all lights are off
UPDATE Day6.LightGrid
SET lightState = 0;

DECLARE @CurrentInstruction VARCHAR(100);

DECLARE instruction_cursor CURSOR LOCAL FAST_FORWARD
FOR
	SELECT Instruction
	FROM Day6.Instructions
	ORDER BY position;

OPEN instruction_cursor
FETCH NEXT FROM instruction_cursor INTO @CurrentInstruction
WHILE @@FETCH_STATUS = 0
BEGIN
	EXEC Day6.LightsInstructionPart1 @CurrentInstruction;

	FETCH NEXT FROM instruction_cursor INTO @CurrentInstruction;
END

CLOSE instruction_cursor;
DEALLOCATE instruction_cursor;

--Get the count of the lights which are turned on
--THIS IS THE RESULT FOR PART 1
SELECT COUNT(*) AS PART1
FROM Day6.LightGrid
WHERE lightState = 1;


/*
RUN THROUGH THE INSTRUCTIONS FOR PART 2
*/
--Before running through the instructions make sure all lights are off
UPDATE Day6.LightGrid
SET lightState = 0;

DECLARE @CurrentInstruction_PART2 VARCHAR(100);

DECLARE instruction_cursor CURSOR LOCAL FAST_FORWARD
FOR
	SELECT Instruction
	FROM Day6.Instructions
	ORDER BY position;

OPEN instruction_cursor
FETCH NEXT FROM instruction_cursor INTO @CurrentInstruction_PART2
WHILE @@FETCH_STATUS = 0
BEGIN
	EXEC Day6.LightsInstructionPart2 @CurrentInstruction_PART2;

	FETCH NEXT FROM instruction_cursor INTO @CurrentInstruction_PART2;
END

CLOSE instruction_cursor;
DEALLOCATE instruction_cursor;

--Get the count of the lights which are turned on
SELECT SUM(lightState) AS PART2
FROM Day6.LightGrid;

select *
from Day6.LightGrid