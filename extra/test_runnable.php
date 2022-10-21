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
}
HERE;

$tokens = PhpToken::tokenize($code);

foreach ($tokens as $token) {
    echo "Line {$token->line}: {$token->getTokenName()}", PHP_EOL;
}