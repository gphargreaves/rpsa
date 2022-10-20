<?php

$code=<<<'HERE'
<?php

class A {
    private $testing = "test";
}
HERE;

$tokens = PhpToken::tokenize($code);

foreach ($tokens as $token) {
    echo "Line {$token->line}: {$token->getTokenName()} ('{$token->text}')", PHP_EOL;
}