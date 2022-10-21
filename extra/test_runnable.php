<?php

$code=<<<'HERE'
<?php

class A {
    private $testing = "test";

    public function testing(string $a): void{
        if($a === 'test'){
            echo 'Hello!';
        }
    }

    public static function testingStr(string $a): void{
        echo 'Do a thing' . $a;
    }
}
HERE;

$tokens = PhpToken::tokenize($code);

foreach ($tokens as $token) {
    echo "Line {$token->line}: {$token->getTokenName()}", PHP_EOL;
}