const text_color_multiplier = 0.45;
const accent_color_multiplier = 0.7;
const saturation_multiplier = 0.8;
const min_color_multiplier = 0.2;
const max_color_multiplier = 0.6; /// Must be > 0.1!!!!
const color_change_speed = 0.9; // Must be between [0,1), doesnt actually change color change :p

class QueryComponentColor {
	private redComponent: number;
	private greenComponent: number;
	private blueComponent: number;
	private degree: number;

	constructor(r:number, g:number, b:number, d: number){
		this.redComponent = r;
		this.greenComponent = g;
		this.blueComponent = b;
		this.degree = d;

		this.normalizeComponents();
	}

	getTextColor(): string{
		let redHex = this.componentToHex(this.redComponent * text_color_multiplier);
		let greenHex = this.componentToHex(this.greenComponent * text_color_multiplier);
		let blueHex = this.componentToHex(this.blueComponent * text_color_multiplier);
		return "#" + redHex + greenHex + blueHex;
	}

	getAccentColor(): string{
		let redHex = this.componentToHex(this.redComponent * accent_color_multiplier);
		let greenHex = this.componentToHex(this.greenComponent * accent_color_multiplier);
		let blueHex = this.componentToHex(this.blueComponent * accent_color_multiplier);
		return "#" + redHex + greenHex + blueHex;
	}

	getBackgroundColor(): string{
		let redHex = this.componentToHex(this.redComponent * saturation_multiplier);
		let greenHex = this.componentToHex(this.greenComponent * saturation_multiplier);
		let blueHex = this.componentToHex(this.blueComponent * saturation_multiplier);
		return "#" + redHex + greenHex + blueHex;
	}

	static createBaseColor(): QueryComponentColor{
		return new QueryComponentColor(255,255,255,1);
	}

	createChildColor(): QueryComponentColor{
		let newRed = this.createNewColorComponent(this.redComponent);
		let newGreen = this.createNewColorComponent(this.greenComponent);
		let newBlue = this.createNewColorComponent(this.blueComponent);
		
		return new QueryComponentColor(newRed,newGreen,newBlue,this.degree+1);
	}

	private normalizeComponents(){
		let r = this.redComponent;
		let g = this.greenComponent;
		let b = this.blueComponent;
		let max = Math.max(r,g,b);
		this.redComponent = clampColor(r + (255-max));
		this.greenComponent = clampColor(g + (255-max));
		this.blueComponent = clampColor(b + (255-max));
	}

	private componentToHex(component_val:number):string{
		let as_integer = clampColor(Math.floor(component_val));
		let as_str = as_integer.toString(16);
		return as_str.length<2? "0"+as_str : as_str ;
	}

	private createNewColorComponent(initial_val:number):number{
		let multiplier = (Math.random() - 0.5)*(max_color_multiplier-0.1)*(Math.pow(color_change_speed,this.degree-1));

		return clampColor(Math.floor(initial_val*(1+multiplier)));		
	}

  };  

function clampColor(value: number):number{
	return value < 0 ? 0 : (value > 255 ? 255 : value); 

}

export {QueryComponentColor};